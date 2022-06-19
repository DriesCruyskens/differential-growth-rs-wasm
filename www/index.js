import * as wasm from "differential-growth";
import * as dat from 'dat.gui';
import * as paper from "paper";
import { saveAs } from "file-saver";

const gui = new dat.GUI();

// wasm.greet();

let totalSteps = 150;
let currentStep = 0;

let radius = 200
let amountOfPoints = 400

// Only executed our code once the DOM is ready.
window.onload = function () {
    paper.setup(document.getElementById("paper-canvas"));

    // initialise path
    let path = new paper.Path();
    path.strokeColor = 'black';
    path.closed = true
    //path.selected = true;

    let line = new wasm.Line(
        paper.view.center.x,
        paper.view.center.y,
        amountOfPoints,
        radius,
        0.2,
        2.0,
        10.0,
        1.1,
        5.0,
    )

    // Loop
    function animationLoop() {
        if (currentStep > totalSteps) {
            return
        }

        line.run()
        
        path.removeSegments()
        path.addSegments(getSegments(line))

        path.smooth()

        currentStep = currentStep + 1
        requestAnimationFrame(animationLoop);
      }
      requestAnimationFrame(animationLoop);
}

function getSegments(line) {
    let segments = []

    let arr = line.export_as_slice();

    for (let i = 0; i < arr.length; i = i + 2) {
        let point = new paper.Point(arr[i], arr[i + 1])
        // let vector = point.subtract(paper.view.center);
        // segments.push(point.add(vector.multiply(Math.random() / 100)))
        segments.push(point)
    }

    return segments
}