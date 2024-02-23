/**
 * Basic render function
 *
 * @type {import("../types").Renderer["renderFrame"]}
 */
export function renderFrame(size, frameInfos, context) {
  context.innerText = `${frameInfos.snake.head.x} - ${frameInfos.snake.head.y}`
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
