import path from 'node:path';

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

console.log(version());
console.log(resolvedFilePathOfGameRecording);
