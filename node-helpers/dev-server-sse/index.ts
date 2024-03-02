import path from 'node:path';
import fs from 'node:fs/promises';
import url from 'url';
import localIpUrl from 'local-ip-url';
import { parseGameStateFromAsyncIterator, InitOptions } from 'snakepipe';

import { makeServer } from './server.js'


const __dirname = url.fileURLToPath(new URL('.', import.meta.url)); // __dirname for esm

if (process.argv.length < 3) {
  console.log("You must pass the path of the file containing the recording of a game.");
  process.exit(64);
}
if (process.argv.length > 3) {
  console.log("Too much arguments passed");
  process.exit(64);
}

const [filePathOfGameRecording] = process.argv.slice(2, 3);

let resolvedFilePathOfGameRecording: string | null = null;

if (path.isAbsolute(filePathOfGameRecording)) {
  resolvedFilePathOfGameRecording = filePathOfGameRecording
} else {
  resolvedFilePathOfGameRecording = path.resolve(process.cwd(), filePathOfGameRecording);
}

main(resolvedFilePathOfGameRecording);

function timeout(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

async function* infiniteAsyncGeneratorFromArrayString(input: Array<string>, delay = 120) {
  let currentIndex = 0;
  yield input[currentIndex];
  while (true) {
    currentIndex++;
    if (input[currentIndex]?.trim()) {
      await timeout(120);
      yield input[currentIndex]
    }
    else {
      // loop back
      currentIndex = 1;
    }
  }
}

function makeInitOptionWithLocalIp(initOptions: InitOptions, port: number): InitOptions {
  const localIp = localIpUrl();
  return {
    ...initOptions,
    metadatas: {
      ...(initOptions.metadatas || {}),
      'render-browser-host': `http://${localIp}:${port}`
    }
  }
}

async function main(resolvedFilePathOfGameRecording: string, port = 8080) {
  const staticFolder = path.resolve(__dirname, '../..', 'static');
  const fileContent =
    (await fs.readFile(resolvedFilePathOfGameRecording))
      .toString()
      .split('\n')
      .filter(Boolean);
  const asyncGenerator = infiniteAsyncGeneratorFromArrayString(fileContent)
  const { options, lines } = await parseGameStateFromAsyncIterator(asyncGenerator);
  const initOptionsWithLocalIp = makeInitOptionWithLocalIp(options, port);
  makeServer({ options: initOptionsWithLocalIp, lines }, staticFolder).listen({ port, host: "0.0.0.0" }).then(() => {
    console.log(`Listening on ${initOptionsWithLocalIp.metadatas?.['render-browser-host']}`);
  });
}
