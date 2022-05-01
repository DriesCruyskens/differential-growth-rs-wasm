import * as wasm from "differential-growth";
import * as dat from 'dat.gui';
import * as paper from "paper";
import { saveAs } from "file-saver";

const gui = new dat.GUI();

// wasm.greet();

// Only executed our code once the DOM is ready.
window.onload = function() {
    paper.setup(document.getElementById("paper-canvas"));
}