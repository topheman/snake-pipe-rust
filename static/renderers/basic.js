/**
 * Basic render function
 *
 * @type {import("../types").Renderer["renderFrame"]}
 */
export function renderFrame(size, frameInfos, context) {
  console.log("basic > renderFrame");
  const buffer = [];
  for (let i = 0; i < size.width; i++) {
    buffer.push(Array.from({ length: size.height }, () => ' '));
  }
  buffer[frameInfos.snake.head.y][frameInfos.snake.head.x] = 'H';
  buffer[frameInfos.fruit.y][frameInfos.fruit.x] = 'F';
  frameInfos.snake.tail.forEach(tailFragment => {
    buffer[tailFragment.y][tailFragment.x] = 'T';
  })
  console.log(buffer);
  const rendered = buffer.map(row => `${row.join('')}`).join('\r\n');
  context.textContent = rendered;
}

/**
 *
 * @type {import("../types").Renderer["setup"]}
 */
export function setup(initOptions, rootNode) {
  console.log("basic > setup");
  const preNode = document.createElement('pre');
  rootNode.appendChild(preNode);
  preNode.style.width = `${initOptions.size.width}ch`;
  preNode.style.height = `calc(${preNode.style.lineHeight}*${initOptions.size.height})`;
  return {
    context: preNode,
    cleanup: () => { }
  };
}
