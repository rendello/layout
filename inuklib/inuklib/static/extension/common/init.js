import init, { tokenize } from './inuklib.js';

function traverse(node, func) {
  if ( node.nodeType === Node.TEXT_NODE
    && node.parentElement
    && getComputedStyle(node.parentElement).display !== 'none'
    && node.nodeValue.trim() !== '') {

    const wrapper = document.createElement('span');
    wrapper.innerHTML = func(node.nodeValue);
    node.parentElement.replaceChild(wrapper, node);
  }

  for (let child of node.childNodes) {
    traverse(child, func);
  }
}

function inuk() {
  const body = document.body;
  traverse(body, tokenize);
}


async function run() {
    await init();
    window.tokenize = tokenize;
    window.inuk = inuk;
}

run();

