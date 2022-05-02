import * as wasm from "differential-growth";
import * as dat from 'dat.gui';
import * as paper from "paper";
import { saveAs } from "file-saver";

const gui = new dat.GUI();

// wasm.greet();

let totalSteps = 500;
let currentStep = 0;

let radius = 100
let amountOfPoints = 40

let rejectionDistance = 80;
let rejectionFactor = 500

let milliSecondsPerFrame = 16;

// Only executed our code once the DOM is ready.
window.onload = function() {
    paper.setup(document.getElementById("paper-canvas"));

    // initialise path
    let path = new paper.Path();
    path.strokeColor = 'black';
    path.addSegments(generateCirlePoints(amountOfPoints, radius))
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

        // introduce new nodes

        // Split edge if it gets too long

        // attraction of connected nodes

        // rejection of nearby nodes
        path.segments.forEach((primarySegment) => {
            let closeSegments = path.segments.filter((segment) => {
                return primarySegment.index != segment.index
                    && segment.index != primarySegment.previous.index
                    && segment.index != primarySegment.next.index
                    && primarySegment.point.getDistance(segment.point, false) < rejectionDistance
            })

            if (closeSegments.length == 0) {
                return;
            }

            let vector = new paper.Point(0, 0)

            closeSegments.map((closeSegment) => closeSegment.point).forEach((point) => {
                let v = primarySegment.point.subtract(point).multiply(1/rejectionFactor)
                vector = vector.add(v)
            })

            primarySegment.point.set(primarySegment.point.x + vector.x, primarySegment.point.y + vector.y)
        })


        // path.segments.forEach((segment) => {
        //     segment.point.set(segment.point.x + 2, segment.point.y +2)
        // })

        currentStep = currentStep + 1
    }, milliSecondsPerFrame)
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
        let point = new paper.Point(x, y)
        let vector = point.subtract(paper.view.center);
        segments.push(point.add(vector.multiply(Math.random()/5)))
    }

    return segments
}