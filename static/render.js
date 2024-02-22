/**
 * Basic render function
 *
 * @param {{
    width: number
    height: number
  }} size
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
function renderFrame(size, frameInfos, rootNode) {
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
  rootNode.textContent = rendered;
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
  node.style.width = `${initOptions.size.width}ch`;
  node.style.height = `calc(${node.style.lineHeight}*${initOptions.size.height})`;
  return initOptions;
}

((global) => {
  const gameScreen = document.getElementById("game-screen");
  let initOptions = null;
  function process(eventName, payload) {
    switch (eventName) {
      case 'connected':
        console.log("connected", payload);
        initOptions = setup(payload, gameScreen);
        break;
      case 'event':
        console.log("event", payload);
        if (initOptions) {
          renderFrame(initOptions.size, payload, gameScreen);
        }
        break
      default:
        console.error(`Usupported "${eventName}" event`);
    }
  }
  global.bootstrap(process);
})(window);
