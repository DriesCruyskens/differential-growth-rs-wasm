import * as wasm from "differential-growth";
import * as dat from 'dat.gui';
import * as paper from "paper";
import { saveAs } from "file-saver";

const gui = new dat.GUI();

// wasm.greet();

let totalSteps = 1000;
let currentStep = 0;

let radius = 100;
let amountOfPoints = 200;

let max_force = 0.2;
let max_speed = 2.0;
let desired_separation = 10.0;
let separation_cohesion_ratio = 1.1;
let max_edge_length = 5.0;


// Only executed our code once the DOM is ready.
window.onload = function () {
    paper.setup(document.getElementById("paper-canvas"));

    // initialise path
    let path = new paper.Path();
    path.strokeColor = 'black';
    path.closed = true
    path.selected = true;

    let line = new wasm.Line(
        paper.view.center.x,
        paper.view.center.y,
        amountOfPoints,
        radius,
        max_force,
        max_speed,
        desired_separation,
        separation_cohesion_ratio,
        max_edge_length,
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