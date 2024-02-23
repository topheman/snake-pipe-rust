import { makeRenderInfos } from './utils.js'

const renderInfos = makeRenderInfos(["version"]);

/**
 * Basic render function
 *
 * @type {import("../types").Renderer["renderFrame"]}
 */
export function renderFrame(initOptions, frameInfos, context) {
  context.gameNode.innerHTML = `
    <li>State: ${frameInfos.state}</li>
    <li>Score: ${frameInfos.score}</li>
    <li>Fruit: x: ${frameInfos.fruit.x} / y: ${frameInfos.fruit.y}</li>
    <li>Snake Head: x: ${frameInfos.snake.head.x} / y: ${frameInfos.snake.head.y}</li>
    <li>Snake Tail:<ul>${frameInfos.snake.tail.map(item => {
    return `<li>x: ${item.x} / y: ${item.y}</li>`
  }).join('')}</ul></li>`
  renderInfos(initOptions, frameInfos, context.infosNode);
}

/**
 *
 * @type {import("../types").Renderer["setup"]}
 */
export function setup(initOptions, rootNode) {
  const gameNode = document.createElement('ul');
  gameNode.id = "inspect-game";
  rootNode.appendChild(gameNode);
  const infosNode = document.createElement('ul');
  infosNode.id = "basic-infos";
  rootNode.appendChild(infosNode);
  return {
    context: {
      gameNode,
      infosNode,
    },
    cleanup: () => { }
  };
}
