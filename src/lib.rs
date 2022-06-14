mod utils;

use micromath::vector;
use utils::{generate_points_of_circle, set_panic_hook};
use wasm_bindgen::prelude::*;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, differential-growth!");
}

#[wasm_bindgen]
pub fn init(
    origin_x: f32,
    origin_y: f32,
    amount_of_points: usize,
    radius: f32,
    // https://rustwasm.github.io/wasm-bindgen/reference/types/boxed-number-slices.html
) -> Box<[f32]> {
    set_panic_hook();

    let line = Line::new(origin_x, origin_y, amount_of_points, radius);

    return line.nodes.into_iter().map(|x| x.position.x).collect::<Vec<f32>>().into_boxed_slice();
}

struct Line {
    nodes: Vec<Node>,
}

impl Line {
    pub fn new(origin_x: f32, origin_y: f32, amount_of_points: usize, radius: f32) -> Line {
        let nodes: Vec<Node> = generate_points_of_circle(origin_x, origin_y, amount_of_points, radius)
            .into_iter()
            .map(|point: vector::F32x2| Node::new(point)).collect();
        Line { nodes }
    }
}

struct Node {
    position: vector::F32x2,
    velocity: vector::F32x2,
    acceleration: vector::F32x2,
    r: f32,
    max_force: f32,
    max_speed: f32,
}

impl Node {
    pub fn new(position: vector::F32x2) -> Node {
        Node {
            position,
            velocity: vector::F32x2 { x: 0.0, y: 0.0 },
            acceleration: vector::F32x2 { x: 0.0, y: 0.0 },
            r: 2.0, max_speed: 2.0, max_force: 0.03
        }
    }
}
