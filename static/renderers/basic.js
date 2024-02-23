import { makeRenderInfos } from './utils.js'

const renderInfos = makeRenderInfos(["score", "version"]);

/**
 * Basic render function
 *
 * @type {import("../types").Renderer["renderFrame"]}
 */
export function renderFrame(initOptions, frameInfos, context) {
  const buffer = [];
  for (let i = 0; i < initOptions.size.height; i++) {
    buffer.push(Array.from({ length: initOptions.size.width }, () => '·'));
  }
  buffer[frameInfos.snake.head.y][frameInfos.snake.head.x] = 'H';
  buffer[frameInfos.fruit.y][frameInfos.fruit.x] = 'F';
  frameInfos.snake.tail.forEach(tailFragment => {
    buffer[tailFragment.y][tailFragment.x] = 'T';
  });
  const rendered = buffer.map(row => `${row.join('')}`).join('\r\n');
  context.preNode.textContent = rendered;
  renderInfos(initOptions, frameInfos, context.infosNode);
}

/**
 *
 * @type {import("../types").Renderer["setup"]}
 */
export function setup(initOptions, rootNode) {
  const preNode = document.createElement('pre');
  preNode.id = "basic-game";
  rootNode.appendChild(preNode);
  const infosNode = document.createElement('ul');
  infosNode.id = "basic-infos";
  rootNode.appendChild(infosNode);
  preNode.style.width = `${initOptions.size.width}ch`;
  preNode.style.height = `calc(${preNode.style.lineHeight}*${initOptions.size.height})`;
  return {
    context: {
      preNode,
      infosNode,
    },
    cleanup: () => { }
  };
}
