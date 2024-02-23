/**
 * Basic render function
 *
 * @type {import("../types").Renderer["renderFrame"]}
 */
export function renderFrame(size, frameInfos, context) {
  context.innerHTML = `<ul>
    <li>State: ${frameInfos.state}</li>
    <li>Score: ${frameInfos.score}</li>
    <li>Fruit: x: ${frameInfos.fruit.x} / y: ${frameInfos.fruit.y}</li>
    <li>Snake Head: x: ${frameInfos.snake.head.x} / y: ${frameInfos.snake.head.y}</li>
    <li>Snake Tail:<ul>${frameInfos.snake.tail.map(item => {
    return `<li>x: ${item.x} / y: ${item.y}</li>`
  }).join('')}</ul></li>
  </ul>`
}

/**
 *
 * @type {import("../types").Renderer["setup"]}
 */
export function setup(initOptions, rootNode) {
  return {
    context: rootNode,
    cleanup: () => { }
  };
}
