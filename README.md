## Differential Line Growth with Rust WASM

This project uses wasm-pack and took the official wasm-pack game of life tutorial as a starting point.

### Notes

js: 
- Datgui
- Export SVG/ parameters
- Statistics
- 2 different renderers: paperjs (smoother animation) and canvas-api (faster)
  
Rust: 
- nalgebra for its point and vector
- Benchmarks using criterionrs

optimisations:

- kd-tree for getting neighbor nodes when calculating rejection forces
- removed branching out of hot loops
- canvas calls in wasm (48s -> 43s)

failed optimisations:

- Letting Javascript access wasm's linear memory for the point's coordinates actually decreases perfomance. That is why I pass a boxed slice.
- Moving requestAnimationFrame to wasm does not improve performance.



### Developing

#### Compile rust (execute manually after file changes):
`wasm-pack build --target web`

> We need to specify `--target web` because we are using Vite which is a no-bundler.
> <https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html>
> <https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html>

#### Start Vite dev server (auto re-compiles on file changes):
`cd www && npm start`

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

### TODOS
- starting shapes dropdown
- draw line using mouse?
- different growing algorithms + datgui dropdown
  
- publish to npm
