import EventEmitter from 'node:events';
import Fastify from 'fastify'
import { FastifySSEPlugin } from "fastify-sse-v2";
import fastifyStatic from '@fastify/static';

import { parseGameStateFromAsyncIterator, Game } from 'snakepipe';

type Input = Awaited<ReturnType<typeof parseGameStateFromAsyncIterator>>

function makeEventEmitterFromAsyncGenerator(lines: () => AsyncGenerator<Game>) {
  const myEmitter = new EventEmitter();

  const clientsConnected = new Set();

  myEmitter.on("connect", (reqId) => {
    clientsConnected.add(reqId);
  });

  myEmitter.on("disconnect", (reqId) => {
    clientsConnected.delete(reqId);
  });

  const iterator = lines();
  (async function () {
    while (true) {
      const nextLine = await iterator.next();
      if (!nextLine.done && nextLine.value) {
        myEmitter.emit("line", nextLine.value)
      }
    }
  })()

  return myEmitter;
}

export function makeServer(input: Input, staticFolder: string) {
  const gameEvents = makeEventEmitterFromAsyncGenerator(input.lines);

  const server = Fastify({
    logger: false
  });

  server.register(fastifyStatic, {
    root: staticFolder
  })

  server.register(FastifySSEPlugin);
  server.get("/events", function (req, res) {
    function listener(line: Game) {
      res.sse({ data: JSON.stringify(line) });
    }
    req.raw.on('close', () => {
      gameEvents.off("line", listener); // if used in production with eavy traffic, consider increasing `emitter.setMaxListeners()` (currently up to 11 clients in parallel)
      res.sseContext.source.end();
    })
    res.sse({ data: "connected" });
    gameEvents.on("line", listener);
  });

  server.get('/init-options', async function handler(request, reply) {
    return input.options;
  });

  return server;
}
