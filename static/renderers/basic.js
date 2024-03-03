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
 * @param {HTMLElement} rootNode
 */
function prepareZoomSlider(rootNode, zoomLevel = { min: 5, max: 24, defaultValue: 16 }) {
  function updateCssZoomValue(value, zoomLevel) {
    if (value >= zoomLevel.min && value <= zoomLevel.max) {
      rootNode.style.setProperty('--basic-zoom-value', value);
      rootNode.style.setProperty('--basic-zoom-font-size', `${value}px`);
      return true;
    }
    return false
  }
  updateCssZoomValue(zoomLevel.defaultValue, zoomLevel);
  const zoomSliderWrapper = document.createElement('div');
  zoomSliderWrapper.id = "zoom-slider-wrapper";
  zoomSliderWrapper.innerHTML = `
  <button type="button" name="decrement">-</button>
  <input type="range" id="zoom-slider" name="zoom-slider" min="${zoomLevel.min}" max="${zoomLevel.max}" step="1" />
  <button type="button" name="increment">+</button>
  `;
  rootNode.appendChild(zoomSliderWrapper);
  const zoomSlider = document.getElementById('zoom-slider');
  zoomSlider.value = rootNode.style.getPropertyValue('--basic-zoom-value');
  zoomSlider.addEventListener('input', function (event) {
    updateCssZoomValue(event.target.value, zoomLevel);
  });
  zoomSliderWrapper.addEventListener('click', (event) => {
    let currentValue = Number(rootNode.style.getPropertyValue('--basic-zoom-value'));
    if (event.target.matches('[name=increment]')) {
      const newValue = currentValue + 1;
      if (updateCssZoomValue(newValue, zoomLevel)) {
        zoomSlider.value = newValue;
      }
    }
    if (event.target.matches('[name=decrement]')) {
      const newValue = currentValue - 1;
      if (updateCssZoomValue(newValue, zoomLevel)) {
        zoomSlider.value = newValue;
      }
    }
  });
}

/**
 *
 * @type {import("../types").Renderer["setup"]}
 */
export function setup(initOptions, rootNode) {
  prepareZoomSlider(rootNode);
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
