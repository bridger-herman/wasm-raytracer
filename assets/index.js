import { importWasm } from './loadWasm.js';

function init() {
  console.log('Hello from JavaScript!');
}

window.onload = () => importWasm().then(init);
