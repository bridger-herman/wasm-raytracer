import { importWasm } from './loadWasm.js';
import { render_scene } from '../pkg/wasm_raytracer.js'

const DEFAULT_SCENE =
"# Simple Sphere Scene\n" +
"camera -6 2 -4 .77 0 .64 0 1 0 35\n" +
"resolution 1024 768\n" +
"\n" +
"# ground sphere\n" +
"material .75 .75 .75 .75 .75 .75 .3 .3 .3 32 .2 .2 .2 1.5\n" +
"sphere 0 -50 0 50\n" +
"\n" +
"# red sphere\n" +
"material 1 0 0 1 0 0 .3 .3 .3 32 .2 .2 .2 1.5\n" +
"sphere -3 1 0 .75\n" +
"\n" +
"# green sphere\n" +
"material 0 .7 0 0 .7 0 0 0 0 16 .9 .9 .9 1.1\n" +
"sphere 0 1.25 0 1\n" +
"\n" +
"# blue sphere\n" +
"material 0 0 1 0 0 1 0 0 0 16 0 0 0 1.0\n" +
"sphere 3 1.5 0 1.25\n" +
"\n" +
"# white overhead light\n" +
"point_light 10 10 10 0 5 0\n" +
"ambient_light .25 .25 .25\n" +
"background .05 .05 .05\n" +
"\n" +
"max_depth 5\n";

document.getElementById('render-scene').addEventListener('click',
    () => {
  let sceneText = document.getElementById('scene-file').value;
  updateSceneFile(sceneText);
});

document.getElementById('upload-scene-file').addEventListener('change',
    (event) => {
  if (!event.target.files || !event.target.files[0]) {
    console.log('No files');
    return;
  }
  let reader = new FileReader();
  let fileContent = '';
  reader.addEventListener('load', (fileEvent) => {
    if (fileEvent.target.readyState != 2 || fileEvent.target.error) {
      console.log('Error loading file');
    } else {
      updateSceneFile(fileEvent.target.result);
      document.getElementById('scene-file').value = fileEvent.target.result;
    }
  });
  reader.readAsBinaryString(event.target.files[0]);
});

function updateSceneFile(sceneText) {
  let b64_bytes = render_scene(sceneText);
  document.getElementById('rendered-scene').src = "data:image/png;base64," + b64_bytes;
}


function init() {
  updateSceneFile(DEFAULT_SCENE);
  document.getElementById('scene-file').value = DEFAULT_SCENE;
}

window.onload = () => importWasm().then(init);
