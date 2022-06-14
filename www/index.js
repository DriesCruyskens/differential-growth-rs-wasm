import * as wasm from "differential-growth";
import * as dat from 'dat.gui';
import * as paper from "paper";
import { saveAs } from "file-saver";

const gui = new dat.GUI();

// wasm.greet();

let totalSteps = 150;
let currentStep = 0;

let radius = 100
let amountOfPoints = 400

let rejectionDistance = 80;
let rejectionFactor = 0.001

let attractionFactor = 0.01;

// Only executed our code once the DOM is ready.
window.onload = function () {
    paper.setup(document.getElementById("paper-canvas"));

    // initialise path
    let path = new paper.Path();
    path.strokeColor = 'black';
    path.addSegments(initSegments(amountOfPoints, radius))
    path.closed = true
    path.smooth()
    path.selected = true;

    console.log(path)

    // Loop
    function repeatOften() {
        console.log(totalSteps, currentStep)
        if (currentStep > totalSteps) {
            return
        }

        // introduce new nodes

        // Split edge if it gets too long
        let curvesToSplit = []
        path.curves.forEach((curve) => {
            if (curve.point1.getDistance(curve.point2) > 40) {
                curvesToSplit.push(curve)
            }
        })

        curvesToSplit.forEach((curve) => {
            curve.divideAtTime(0.5)
        })

        // attraction of connected nodes
        path.segments.forEach((primarySegment, i) => {
            let lastIndex = path.segments.length - 1
            let previousSegment = i == 0 ? path.segments[lastIndex] : path.segments[i - 1]
            let nextSegment = i == lastIndex ? path.segments[0] : path.segments[i + 1]

            let vector = new paper.Point(0, 0)
            if (primarySegment.point.getDistance(nextSegment.point) > 5) {
                let v1 = primarySegment.point.subtract(nextSegment.point).multiply(attractionFactor)
                vector.add(v1)
            }
            if (primarySegment.point.getDistance(previousSegment.point) > 5) {
                let v2 = primarySegment.point.subtract(previousSegment.point).multiply(attractionFactor)
                vector.add(v2)
            }

            primarySegment.point.set(primarySegment.point.x + vector.x, primarySegment.point.y + vector.y)
        })

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
                let v = primarySegment.point.subtract(point).multiply(rejectionFactor)
                vector = vector.add(v)
            })

            primarySegment.point.set(primarySegment.point.x + vector.x, primarySegment.point.y + vector.y)
        })


        // path.segments.forEach((segment) => {
        //     segment.point.set(segment.point.x + 2, segment.point.y +2)
        // })

        currentStep = currentStep + 1
        requestAnimationFrame(repeatOften);
      }
      requestAnimationFrame(repeatOften);
}

// Generates the point of a circle
// https://www.mathopenref.com/coordcirclealgorithm.html
function initSegments(amountOfPoints, r) {
    let segments = []

    let arr = wasm.init(
        paper.view.center.x,
        paper.view.center.y,
        amountOfPoints,
        r
    )

    for (let i = 0; i < arr.length; i = i + 2) {
        let point = new paper.Point(arr[i], arr[i + 1])
        let vector = point.subtract(paper.view.center);
        segments.push(point.add(vector.multiply(Math.random() / 100)))
    }

    return segments
}