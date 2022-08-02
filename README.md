## Differential Line Growth with Rust WASM

A WebAssembly wrapper around a Rust implementation of the differential growth algorithm.

![example gif](https://raw.githubusercontent.com/DriesCruyskens/differential-growth-rs-wasm/main/images/example.gif)

As you can see its blazingly fast.

### Example

A full example available online [here](https://differential-growth-example.vercel.app/)(and its [repo](https://github.com/DriesCruyskens/differential-growth-example)) and is provided in the source code's `/www` directory. 
Run it using:

```js
cd www && npm init && npm start
```

You can play around with the input variables using [dat.gui](https://github.com/dataarts/dat.gui) and export the result to SVG.

### Usage

The package is build using `wasm-pack build --target web` so it can be used with native javascript modules instead of needing a bundler. More info in the `wasm-bindgen` docs
 [here](https://rustwasm.github.io/wasm-bindgen/reference/deployment.html) and
 [here](https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html), 
 and in the `wasm-pack` docs [here](https://rustwasm.github.io/docs/wasm-pack/commands/build.html?highlight=--target#target).
 This is done in order to enable development using [Vite](https://v2.vitejs.dev/).

At the time of development WASM and Javascript can only communicate using basic types. The array of point coordinates is represented as a flattened array of numbers when it is returned or passed as a function argument. Manual conversion to and from an array of points is necessary.


```js
// Since we use `--target web`, manual initialisation of the WebAssembly module is required.
import init, {
  DifferentialGrowthWasm,
  generate_points_on_circle,
} from "differential-growth-rs-wasm";

// Initialise WASM
await init();

// Using the included helper function to generate point on a circle.
let starting_points = generate_points_on_circle(
        200, // origin_x
        200, // origin_y
        10.0, // radius
        10 // amount of points
      );

// Instatiate differential growth object
let rustDifferentialGrowth = new DifferentialGrowthWasm(
    starting_points,
    1,5, // maxForce
    1.0, // maxSpeed
    14, // desiredSeparation
    1.1, // separationCohesionRatio
    5.0 // maxEdgeLength
);

// Advance the algorithm by a single iteration.
rustDifferentialGrowth.tick();

// Either let WASM render the result to a canvas.
let canvas = document.getElementById("canvas");
let ctx = canvas.getContext('2d');
rustDifferentialGrowth.render(
    ctx,
    canvas.width,
    canvas.height,
);

// or get the array of coordinates and render yourself.
// don't forget to convert array of numbers to array of vectors.
let points = rustDifferentialGrowth.export_as_slice();      
```

The choice of algorithm parameters is a very important factor to achieving a desirable result.


### Developing

#### Compile rust (execute manually after file changes):
In the project root:
```bash
wasm-pack build --target web
```

#### Run development server
In the `/www` folder:
```bash
npm init
```

```bash
npm start
```

### References
 
- <https://rustwasm.github.io/docs/book/game-of-life/implementing.html>
- <https://rustwasm.github.io/docs/wasm-bindgen/examples/index.html>
- <http://www.codeplastic.com/2017/07/22/differential-line-growth-with-processing/>
- <https://processing.org/examples/flocking.html>
- <https://inconvergent.net/2016/shepherding-random-growth/>
- <https://inconvergent.net/generative/differential-line/>
- <http://www.dgp.toronto.edu/~karan/artexhibit/mazes.pdf>
- <https://rustwasm.github.io/docs/book/game-of-life/time-profiling.html>
- <https://stackoverflow.com/questions/50721411/how-to-see-rust-source-code-when-debugging-webassembly-in-a-browser>
- <https://web.dev/canvas-hidipi/>
- <https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html>
- <https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/packaging-and-publishing.html>

### Notes
failed optimisations:

- Letting Javascript access wasm's linear memory for the point's coordinates actually decreases perfomance. That is why I pass a boxed slice.
- Moving requestAnimationFrame to wasm does not improve performance.

