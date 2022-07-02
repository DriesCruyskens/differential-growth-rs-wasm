## Differential Line Growth with Rust WASM

This project uses wasm-pack and took the official wasm-pack game of life tutorial as a starting point.

### Features

js: 
- Datgui
  
Rust: 
- nalgebra for its point and vector
- Benchmarks using criterionrs

optimisations:

- kd-tree for getting neighbor nodes when calculating rejection forces
- removed branching out of hot loops
- Letting Javascript access wasm's linear memory for the point's coordinates actually decreases perfomance. That is why I pass a boxed slice.

### Developing

Compile rust (execute manually after file changes):
`wasm-pack build`

Start dev server (auto re-compiles on file changes):
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

### TODOS
- starting shapes dropdown
- different growing algorithms + datgui dropdown
- maybe switch from wasm-pack to Trunk https://trunkrs.dev/assets/. Yew is not suited because I want to use DatGui anyway
- move as much as possible to rust: canvas calls, request animation frame

