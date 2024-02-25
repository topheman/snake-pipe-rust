/**
 * This is a simplified version of the source code for `snakepipe-node validate`
 *
 * https://github.com/topheman/snake-pipe-node/blob/master/src/validate.ts
 * https://github.com/topheman/snake-pipe-node/blob/master/src/common.ts
 *
 * See https://github.com/topheman/snake-pipe-rust/issues/25
 */
const readline = require('node:readline');
const { stdin } = require('node:process');

function makeWriteLine(stdout) {
  if (stdout.isTTY) {
    return function writeLineToTTY(str) {
      process.stdout.write(`${str}\n`);
      process.stdout.cursorTo(0);
    };
  } else {
    return function writeLine(str) {
      console.log(str);
    };
  }
}

async function main() {
  const readStdin = readline.createInterface({ input: stdin });
  const writeLine = makeWriteLine(process.stdout);

  const stdinIterator = readStdin[Symbol.asyncIterator]();
  const options = JSON.parse((await stdinIterator.next()).value);

  writeLine(JSON.stringify(options));
  for await (const line of stdinIterator) {
    const parsedLine = JSON.parse(line);
    writeLine(JSON.stringify(parsedLine));
  }
}

main();
