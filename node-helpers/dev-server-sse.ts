import path from 'node:path';
import fs from 'node:fs/promises';

import { parseGameStateFromAsyncIterator, version } from 'snakepipe';

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

async function main(resolvedFilePathOfGameRecording: string) {
  const fileContent =
    (await fs.readFile(resolvedFilePathOfGameRecording))
      .toString()
      .split('\n')
      .filter(Boolean);
  const asyncGenerator = infiniteAsyncGeneratorFromArrayString(fileContent)
  const { options, lines } = await parseGameStateFromAsyncIterator(asyncGenerator);
  console.log(JSON.stringify(options));
  for await (const line of lines()) {
    console.log(JSON.stringify(line));
  }
}
