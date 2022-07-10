import * as wasm from "differential-growth";
import * as dat from "dat.gui";
import * as paper from "paper";
import { saveAs } from "file-saver";
import * as hash from 'object-hash';

// Only executed our code once the DOM is ready.
window.onload = function () {
  const d = new DifferentialGrowth();
  d.run();
};

class DifferentialGrowth {
  constructor() {
    this.params = {
      maxSteps: 2000,

      renderer: "paper-js", // 'paper-js', 'canvas-api'

      strokeWidth: 1,

      statistics: true,

      debug: false,
      smoothPath: false,

      radius: 10,
      nStartingPoints: 10,

      maxForce: 1.5,
      maxSpeed: 1.0,
      desiredSeparation: 14,
      separationCohesionRatio: 1.1,
      maxEdgeLength: 5.0,
    };

    this.canvas = document.getElementById("canvas");
    this.ctx = canvas.getContext("2d");

    paper.setup(canvas);

    // initialise path
    this.path = new paper.Path();
    this.path.strokeColor = "black";
    this.path.closed = true;
    this.path.selected = this.params.debug;

    // Providing 'this' to the animationLoop() function because it is
    // otherwise not available.
    // https://stackoverflow.com/a/49197824
    this.bindedAnimationLoop = this.animationLoop.bind(this);

    this.gui = new dat.GUI({
      name: "Differential Growth",
      autoPlace: true,
      closeOnTop: false,
    });

    this.initGui();

    // init statistics
    this.statistics = document.getElementById("statistics");
    this.frames = [];
    this.lastFrameTimeStamp = performance.now();
  }

  run() {
    this.reset();
    requestAnimationFrame(this.bindedAnimationLoop);
  }

  reset() {
    this.startTimer();

    this.path.selected = this.params.debug;
    this.path.strokeWidth = this.params.strokeWidth;

    this.ctx.lineWidth = this.params.strokeWidth;

    this.differentialGrowth = new wasm.DifferentialGrowth(
      paper.view.center.x,
      paper.view.center.y,
      this.params.nStartingPoints,
      this.params.radius,
      this.params.maxForce,
      this.params.maxSpeed,
      this.params.desiredSeparation,
      this.params.separationCohesionRatio,
      this.params.maxEdgeLength
    );

    this.ctx.clearRect(0, 0, canvas.width, canvas.height);
    this.currentStep = 0;
    this.amountOfTotalPoints = 0;
  }

  renderWithPaperjs() {
    this.path.removeSegments();

    let segments = [];

    for (let i = 0; i < this.pointsArray.length; i = i + 2) {
      let point = new paper.Point(this.pointsArray[i], this.pointsArray[i + 1]);
      segments.push(point);
    }

    this.path.addSegments(segments);

    if (this.smoothPath) {
      this.path.smooth();
    }
  }

  renderWithCanvasApi() {
    this.ctx.clearRect(0, 0, canvas.width, canvas.height);

    this.ctx.beginPath();
    this.strokeStyle = "black";
    this.ctx.moveTo(this.pointsArray[0], this.pointsArray[1]);

    for (let i = 2; i < this.pointsArray.length; i = i + 2) {
      this.ctx.lineTo(this.pointsArray[i], this.pointsArray[i + 1]);
    }

    this.ctx.closePath();
    this.ctx.stroke();
  }

  animationLoop() {
    if (this.currentStep > this.params.maxSteps) {
      return;
    }

    // The run() function returns the array of points as a 1d
    // array of f64's. To get a coordinates for a specific index:
    // x = arr[i*2]; y = arr[i*2 + 1]
    this.pointsArray = this.differentialGrowth.tick();

    if (this.params.renderer == "canvas-api") {
      this.renderWithCanvasApi();
    } else if (this.params.renderer == "paper-js") {
      this.renderWithPaperjs();
    }

    if (this.params.statistics) {
      this.renderStatistics();
    }

    this.amountOfTotalPoints = this.pointsArray.length / 2;

    this.currentStep = this.currentStep + 1;
    requestAnimationFrame(this.bindedAnimationLoop);
  }

  stop() {
    this.currentStep = this.params.maxSteps + 1;
  }

  initGui() {
    this.gui.add(this, "run").name("Run");

    this.gui.add(this, "stop").name("Stop");


    const esthetics = this.gui.addFolder("Esthetics");

    esthetics
      .add(this.params, "renderer", ["canvas-api", "paper-js"])
      .listen()
      .onChange((_) => {
        this.run();
      });

    esthetics
      .add(this.params, "strokeWidth", 1, 10)
      .step(1)
      .listen()
      .onChange((_) => {
        this.run();
      });

    esthetics
      .add(this.params, "debug")
      .listen()
      .onChange((_) => {
        this.run();
      });

    esthetics
      .add(this.params, "smoothPath")
      .listen()
      .onChange((_) => {
        this.run();
      });

    const control = this.gui.addFolder("Control");

    control
      .add(this.params, "maxSteps", 0, 10000)
      .listen()
      .onChange((_) => {
        this.run();
      });

    control
      .add(this.params, "radius", 0, 300)
      .listen()
      .onChange((_) => {
        this.run();
      });

    control
      .add(this.params, "nStartingPoints", 0, 100)
      .listen()
      .onChange((_) => {
        this.run();
      });

    control
      .add(this.params, "maxForce", 0, 2)
      .listen()
      .onChange((_) => {
        this.run();
      });

    control
      .add(this.params, "maxSpeed", 0, 2)
      .listen()
      .onChange((_) => {
        this.run();
      });

    control
      .add(this.params, "desiredSeparation", 0, 50)
      .listen()
      .onChange((_) => {
        this.run();
      });

    control
      .add(this.params, "separationCohesionRatio", 0, 2)
      .listen()
      .onChange((_) => {
        this.run();
      });

    control
      .add(this.params, "maxEdgeLength", 0, 30)
      .listen()
      .onChange((_) => {
        this.run();
      });

    const utilities = this.gui.addFolder("Utilities");

    utilities
      .add(this.params, "statistics")
      .listen()
      .onChange((_) => {
        this.run();
        document.getElementById("statistics").innerHTML = "";
      });

    utilities.add(this, "exportParameters").name("Export parameters");

    utilities.add(this, "exportSVG").name("Export SVG");
  }

  exportParameters() {
    const blob = new Blob([JSON.stringify(this.params)], { type: 'application/json' });
    saveAs(blob, "differential_line_growth_params_" + hash(this.params) + '_' + new Date(Date.now()).toDateString()  + ".json");
  }

  exportSVG() {
    this.renderWithPaperjs();
    const svg = paper.project.exportSVG({ asString: true });
    const blob = new Blob([svg], { type: "image/svg+xml;charset=utf-8" });
    saveAs(blob, "differential_line_growth_" + hash(this.params) + '_' + new Date(Date.now()).toDateString() + ".svg");
  }

  // https://rustwasm.github.io/docs/book/game-of-life/time-profiling.html
  renderStatistics() {
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
    this.statistics.textContent = `
  Frames per Second:
           latest = ${Math.round(fps)}
  avg of last 100 = ${Math.round(mean)}
  min of last 100 = ${Math.round(min)}
  max of last 100 = ${Math.round(max)}

Amount of Steps = ${this.currentStep}
Total amount of Points = ${this.amountOfTotalPoints}
Time elapsed = ${this.endTimer()}s

  `.trim();
  }

  startTimer() {
    this.startTime = new Date();
  }

  endTimer() {
    this.endTime = new Date();
    var timeDiff = this.endTime - this.startTime; //in ms
    // strip the ms
    timeDiff /= 1000;

    // get seconds
    var seconds = Math.round(timeDiff);
    return seconds;
  }
}
