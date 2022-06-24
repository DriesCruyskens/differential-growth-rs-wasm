import * as wasm from "differential-growth";
import * as dat from "dat.gui";
import * as paper from "paper";
import { saveAs } from "file-saver";

const gui = new dat.GUI({
  name: "Differential Line",
  autoPlace: true,
  closeOnTop: false,
});

let params = {
  totalSteps: 300,

  debug: false,
  smoothPath: false,

  radius: 10,
  amountOfPoints: 10,

  maxForce: 0.9,
  maxSpeed: 1.0,
  desiredSeparation: 9.0,
  separationCohesionRatio: 1.1,
  maxEdgeLength: 5.0,
};

let currentStep = 0;

let line;

// Only executed our code once the DOM is ready.
window.onload = function () {
  paper.setup(document.getElementById("paper-canvas"));

  // initialise path
  let path = new paper.Path();
  path.strokeColor = "black";
  path.closed = true;

  params.reset = function() {
    path.selected = params.debug;

    line = new wasm.Line(
      paper.view.center.x,
      paper.view.center.y,
      params.amountOfPoints,
      params.radius,
      params.maxForce,
      params.maxSpeed,
      params.desiredSeparation,
      params.separationCohesionRatio,
      params.maxEdgeLength
    );

    currentStep = 0;
    requestAnimationFrame(animationLoop);
  }

  params.reset();
  initGui();

  // Loop
  function animationLoop() {
    fps.render();

    if (currentStep > params.totalSteps) {
      return;
    }

    path.removeSegments();
    path.addSegments(getSegments(line.run()));

    if (params.smoothPath) {
      path.smooth();
    }

    currentStep = currentStep + 1;
    requestAnimationFrame(animationLoop);
  }

  function getSegments(arr) {
    let segments = [];

    for (let i = 0; i < arr.length; i = i + 2) {
      let point = new paper.Point(arr[i], arr[i + 1]);
      segments.push(point);
    }

    return segments;
  }

  function initGui() {
    gui.add(params, "reset").name("Run");

    gui
      .add(params, "totalSteps", 0, 1000)
      .listen()
      .onChange((value) => {
        params.totalSteps = value;
        params.reset();
      });

    gui
      .add(params, "debug")
      .listen()
      .onChange((value) => {
        params.debug = value;
        params.reset();
      });

    gui
      .add(params, "smoothPath")
      .listen()
      .onChange((value) => {
        params.smoothPath = value;
        params.reset();
      });

    gui
      .add(params, "radius", 0, 300)
      .listen()
      .onChange((value) => {
        params.radius = value;
        params.reset();
      });

    gui
      .add(params, "amountOfPoints", 0, 100)
      .listen()
      .onChange((value) => {
        params.amountOfPoints = value;
        params.reset();
      });

    gui
      .add(params, "maxForce", 0, 2)
      .listen()
      .onChange((value) => {
        params.maxForce = value;
        params.reset();
      });

    gui
      .add(params, "maxSpeed", 0, 2)
      .listen()
      .onChange((value) => {
        params.maxSpeed = value;
        params.reset();
      });

    gui
      .add(params, "desiredSeparation", 0, 50)
      .listen()
      .onChange((value) => {
        params.desiredSeparation = value;
        params.reset();
      });

    gui
      .add(params, "separationCohesionRatio", 0, 2)
      .listen()
      .onChange((value) => {
        params.separationCohesionRatio = value;
        params.reset();
      });

    gui
      .add(params, "maxEdgeLength", 0, 30)
      .listen()
      .onChange((value) => {
        params.maxEdgeLength = value;
        params.reset();
      });
  }
};

// https://rustwasm.github.io/docs/book/game-of-life/time-profiling.html
const fps = new (class {
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
    const fps = (1 / delta) * 1000;

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
})();
