import * as wasm from "differential-growth";
import * as dat from 'dat.gui';
import * as paper from "paper";
import { saveAs } from "file-saver";

const gui = new dat.GUI();

// wasm.greet();

let totalSteps = 300;
let currentStep = 0;

let radius = 10;
let amountOfPoints = 10;

let max_force = 0.9;
let max_speed = 1.0;
let desired_separation = 9.0;
let separation_cohesion_ratio = 1.1;
let max_edge_length = 5.0;


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
        max_force,
        max_speed,
        desired_separation,
        separation_cohesion_ratio,
        max_edge_length,
    )

    // Loop
    function animationLoop() {
        fps.render();

        if (currentStep > totalSteps) {
            return
        }

        line.run()
        
        path.removeSegments()
        path.addSegments(getSegments(line))

        // path.smooth()

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
        // if (isNaN(point.x)) {
        //     debugger;
        // }
        // let vector = point.subtract(paper.view.center);
        // segments.push(point.add(vector.multiply(Math.random() / 100)))
        segments.push(point)
    }

    return segments
}

const fps = new class {
    constructor() {
      this.fps = document.getElementById("fps");
      this.frames = [];
      this.lastFrameTimeStamp = performance.now();
    }
  
    render() {
      // Convert the delta time since the last frame render into a measure
      // of frames per second.
      const now = performance.now();
      const delta = now - this.lastFrameTimeStamp;
      this.lastFrameTimeStamp = now;
      const fps = 1 / delta * 1000;
  
      // Save only the latest 100 timings.
      this.frames.push(fps);
      if (this.frames.length > 100) {
        this.frames.shift();
      }
  
      // Find the max, min, and mean of our 100 latest timings.
      let min = Infinity;
      let max = -Infinity;
      let sum = 0;
      for (let i = 0; i < this.frames.length; i++) {
        sum += this.frames[i];
        min = Math.min(this.frames[i], min);
        max = Math.max(this.frames[i], max);
      }
      let mean = sum / this.frames.length;
  
      // Render the statistics.
      this.fps.textContent = `
  Frames per Second:
           latest = ${Math.round(fps)}
  avg of last 100 = ${Math.round(mean)}
  min of last 100 = ${Math.round(min)}
  max of last 100 = ${Math.round(max)}

Amount of Steps = ${currentStep}
  `.trim();
    }
  };