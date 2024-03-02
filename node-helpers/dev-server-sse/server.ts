import Fastify from 'fastify'
import { FastifySSEPlugin } from "fastify-sse-v2";
import fastifyStatic from '@fastify/static';

import { parseGameStateFromAsyncIterator } from 'snakepipe';

type Input = Awaited<ReturnType<typeof parseGameStateFromAsyncIterator>>

export function makeServer(input: Input, staticFolder: string) {
  const server = Fastify({
    logger: false
  });

  server.register(fastifyStatic, {
    root: staticFolder
  })

  const loop: Record<string, boolean> = {};

  server.register(FastifySSEPlugin);
  /**
   * For the moment, only supports one client at a time
   * If more than one connects at the same time, the iterator will
   * move by more than once at a time (and you will drop frames)
   *
   * It's enough for the moment for a dev-server
   */
  server.get("/events", async function (req, res) {
    loop[req.id] = true;
    req.raw.on('close', () => {
      loop[req.id] = false;
      res.sseContext.source.end();
    })
    res.sse({ data: "connected" });
    const iterator = input.lines();
    while (loop[req.id]) {
      const nextLine = await iterator.next();
      if (!nextLine.done && nextLine.value) {
        res.sse({ data: JSON.stringify(nextLine.value) });
      }
    }
  });

  server.get('/init-options', async function handler(request, reply) {
    return input.options;
  });

  return server;
}
