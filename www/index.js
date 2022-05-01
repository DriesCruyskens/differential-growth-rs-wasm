import * as wasm from "differential-growth";
import * as dat from 'dat.gui';
import * as paper from "paper";
import { saveAs } from "file-saver";

const gui = new dat.GUI();

// wasm.greet();

let totalSteps = 100;
let currentStep = 0;

// Only executed our code once the DOM is ready.
window.onload = function() {
    paper.setup(document.getElementById("paper-canvas"));

    // initialise path
    let path = new paper.Path();
    path.strokeColor = 'black';
    path.addSegments(generateCirlePoints(10, 100))
    path.closed = true
    path.smooth()
    path.selected = true;

    console.log(path)

    // Loop
    let intervalId
    intervalId = setInterval(() => {
        if (currentStep > totalSteps) {
            clearInterval(intervalId)
        }

        // path.segments.forEach((segment) => {
        //     segment.point.set(segment.point.x + 2, segment.point.y +2)
        // })

        currentStep = currentStep + 1
    }, 16)
}

// Generates the point of a circle
// https://www.mathopenref.com/coordcirclealgorithm.html
function generateCirlePoints(amountOfPoints, r) {
    let segments = []

    let h = paper.view.center.x
    let k = paper.view.center.y

    for (let theta = 0; theta < 2*Math.PI; theta = theta + (2 * Math.PI / amountOfPoints)) {
        let x = h + r * Math.cos(theta)
        let y = k + r * Math.sin(theta)
        segments.push(new paper.Point(x, y))
    }

    return segments
}