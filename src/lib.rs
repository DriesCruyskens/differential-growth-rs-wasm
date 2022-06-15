mod utils;

use micromath::{vector::Vector2d, F32};
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

    let line = Line::new(
        F32 { 0: origin_x },
        F32 { 0: origin_y },
        amount_of_points,
        F32 { 0: radius },
    );

    return line
        .nodes
        .into_iter()
        .map(|x| x.position.x.0)
        .collect::<Vec<f32>>()
        .into_boxed_slice();
}

struct Line {
    nodes: Vec<Node>,
}

impl Line {
    pub fn new(origin_x: F32, origin_y: F32, amount_of_points: usize, radius: F32) -> Line {
        let nodes: Vec<Node> =
            generate_points_of_circle(origin_x, origin_y, amount_of_points, radius)
                .into_iter()
                .map(|point: Vector2d<F32>| Node::new(point))
                .collect();
        Line { nodes }
    }
}

struct Node {
    position: Vector2d<F32>,
    velocity: Vector2d<F32>,
    acceleration: Vector2d<F32>,
    r: F32,
    max_force: F32,
    max_speed: F32,
}

impl Node {
    pub fn new(position: Vector2d<F32>) -> Node {
        Node {
            position,
            velocity: Vector2d {
                x: F32 { 0: 0.0 },
                y: F32 { 0: 0.0 },
            },
            acceleration: Vector2d {
                x: F32 { 0: 0.0 },
                y: F32 { 0: 0.0 },
            },
            r: F32 { 0: 2.0 },
            max_speed: F32 { 0: 2.0 },
            max_force: F32 { 0: 0.03 },
        }
    }
}
