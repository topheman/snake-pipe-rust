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

  server.register(FastifySSEPlugin);
  server.get("/events", function (req, res) {
    console.log(req);
    res.sse(
      (async function* () {
        yield { data: "connected" };
        for await (const line of input.lines()) {
          yield { data: JSON.stringify(line) };
        }
      })()
    );
  });

  server.get('/init-options', async function handler(request, reply) {
    return input.options;
  });

  return server;
}
