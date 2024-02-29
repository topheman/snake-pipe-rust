/**
 *
 * @param {"loading" | "connected" | "error"} state
 * @param {string} label
 * @param {HTMLElement} rootNode
 */
function createNodeInsideRootNode(label, rootNode) {
  const loadingNode = document.createElement('p');
  loadingNode.innerHTML = label;
  rootNode.appendChild(loadingNode);
  return null;
}

/**
 *
 * @param {"loading" | "connected" | "ready" | "error"} state
 * @param {HTMLElement} rootNode
 * @param {String?} rendererName
 * @returns {HTMLElement | null}
 */
function prepareRootNode(state, rootNode, rendererName) {
  switch (state) {
    case "loading": {
      return createNodeInsideRootNode("Loading ...", rootNode);
    }
    case "connected": {
      return createNodeInsideRootNode("Connecting ...", rootNode);
    }
    case "error": {
      return createNodeInsideRootNode("An error occured, please reload", rootNode);
    }
    case "ready": {
      rootNode.replaceChildren();
      const gameNode = document.createElement('div');
      gameNode.id = rendererName
      rootNode.appendChild(gameNode);
      return gameNode;
    }
  }
}

async function fetchInitOptions() {
  try {
    const res = await fetch('/init-options');
    if (res.ok) {
      return await res.json();
    }
  }
  catch (e) {
    console.error(e);
    return null;
  }
  return null;
}

/**
 *
 * @param {(eventName: 'connected' | 'event', payload: any) => void} cb
 */
async function bootstrap(cb) {
  const events = new EventSource("/events");
  events.onmessage = (event) => {
    if (event.data === 'connected') {
      fetchInitOptions().then(initOptions => {
        if (initOptions.metadatas['render-browser-host']) {
          document.querySelector('qrcode-display').setAttribute('data', initOptions.metadatas['render-browser-host'])
        }
        cb('connected', initOptions);
      })
    }
    else {
      cb('event', JSON.parse(event.data));
    }
  }
}

/**
 *
 * @type {Record<string, import("./types").Renderer>}
 */
const renderers = {};

/**
 *
 * @returns {string}
 */
function getRendererName() {
  const rendererSwitcher = document.getElementById('renderer-switcher');
  const rendererName = rendererSwitcher.rendererName.value;
  return rendererName
}

/**
 *
 * @returns {import("./types").Renderer}
 */
async function getRenderer() {
  const rendererName = getRendererName();
  if (renderers[rendererName]) {
    return renderers[rendererName];
  }
  renderers[rendererName] = await import(`/renderers/${rendererName}.js`);
  return renderers[rendererName];
}

/**
 * @typedef {import("./types").Renderer} Renderer
 * @param {(renderer: Renderer) => {}} cb
 */
function onUpdateRender(cb) {
  [...document.querySelectorAll('[name=rendererName]')].forEach(node => {
    node.addEventListener('change', async () => {
      cb(await getRenderer());
    })
  })
}

/**
 * @param {import("./types").SetupFunction} setup
 * @param {import("./types").RenderFrameFunction} renderFrame
 */
async function prepareGame() {
  const rootNode = document.getElementById('root');
  prepareRootNode("loading", rootNode);
  /** @type {HTMLElement} */
  let gameNode = null;
  /** @type {import("./types").Renderer | null} */
  let currentRenderer = null;
  let currentRendererContext = null;
  let currentCleanupFunction = null;
  /** @type {import("./types").InitOptions | null} */
  let currentInitOptions = null;

  /**
   *
   * @param {import("./types").Renderer} renderer
   */
  function updateRenderer(renderer) {
    if (currentCleanupFunction) {
      currentCleanupFunction();
    }
    currentRenderer = renderer;
    if (currentInitOptions) {
      gameNode = prepareRootNode('ready', rootNode, getRendererName());
      const { cleanup, context } = currentRenderer.setup(currentInitOptions, gameNode);
      currentRendererContext = context;
      currentCleanupFunction = cleanup
    }

  }
  currentRenderer = await getRenderer();
  updateRenderer(currentRenderer);
  onUpdateRender(updateRenderer);

  function process(eventName, payload) {
    switch (eventName) {
      case 'connected':
        console.log("connected", payload);
        currentInitOptions = payload;
        gameNode = prepareRootNode('ready', rootNode, getRendererName());
        const { cleanup, context } = currentRenderer.setup(currentInitOptions, gameNode);
        currentRendererContext = context;
        currentCleanupFunction = cleanup
        break;
      case 'event':
        console.log("event", payload);
        if (currentInitOptions) {
          currentRenderer.renderFrame(currentInitOptions, payload, currentRendererContext);
        }
        break
      default:
        console.error(`Usupported "${eventName}" event`);
    }
  }
  bootstrap(process);
}

prepareGame();
