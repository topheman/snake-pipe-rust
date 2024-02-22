/**
 *
 * @param {{
  snake: {
    direction: string
    head: {
      x: number
      y: number
    }
    tail: {
      x: number
      y: number
    }[]
  }
  fruit: Fruit
  score: number
  state: string
}} frameInfos
 * @param {HTMLElement} rootNode
 */
function renderFrame(frameInfos, rootNode) {
  let node = document.createElement("p");
  let time = new Date().toLocaleTimeString();
  node.innerText = `${time} - x: ${frameInfos.snake.head.x} / y: ${frameInfos.snake.head.y}`
  rootNode.insertAdjacentElement("afterbegin", node);
}

/**
 *
 * @param {{
  frameDuration: number
  size: {
    width: number
    height: number
  }
  featuresWithVersion: Record<string, string>
  metadatas: Record<string, string>
}} initOptions
 * @param {HTMLElement} node
 */
function setup(initOptions, node) {
  let time = new Date().toLocaleTimeString();
  node.innerText = `${time} - Size: ${initOptions.size.width}x${initOptions.size.height}`
}

((global) => {
  const rootNode = document.getElementById("root");
  const optionsNode = document.getElementById("options");
  function process(eventName, payload) {
    switch (eventName) {
      case 'connected':
        console.log("connected", payload);
        setup(payload, optionsNode);
        break;
      case 'event':
        console.log("event", payload);
        renderFrame(payload, rootNode);
        break
      default:
        console.error(`Usupported "${eventName}" event`);
    }
  }
  global.bootstrap(process);
})(window);
