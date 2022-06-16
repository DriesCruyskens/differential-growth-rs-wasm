mod utils;

use std::ops::{AddAssign, MulAssign, Sub};

use nalgebra::Vector2;
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

    let line: Line = Line::new(
        origin_x,
        origin_y,
        amount_of_points,
        radius,
        0.2,
        2.0,
        10.0,
        1.1,
        5.0,
    );

    return line
        .nodes
        .into_iter()
        .map(|node| node.position.y)
        .collect::<Vec<f32>>()
        .into_boxed_slice();
}

struct Line {
    nodes: Vec<Node>,
    max_force: f32,
    max_speed: f32,
    desired_separation: f32,
    sq_desired_separation: f32,
    separation_cohesionRation: f32,
    max_edge_len: f32,
}

impl Line {
    pub fn new(
        origin_x: f32,
        origin_y: f32,
        amount_of_points: usize,
        radius: f32,
        max_force: f32,
        max_speed: f32,
        desired_separation: f32,
        separation_cohesion_ration: f32,
        max_edge_len: f32,
    ) -> Line {
        let nodes: Vec<Node> =
            generate_points_of_circle(origin_x, origin_y, amount_of_points, radius)
                .into_iter()
                .map(|point: Vector2<f32>| Node::new(point, max_speed, max_force))
                .collect();
        Line {
            nodes,
            max_force,
            max_speed,
            desired_separation,
            sq_desired_separation: desired_separation.sqrt(),
            separation_cohesionRation: separation_cohesion_ration,
            max_edge_len,
        }
    }
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn add_node_at(&mut self, node: Node, index: usize) {
        self.nodes.insert(index, node);
    }

    pub fn run(&mut self) {

    }

    pub fn differentiate() {

    }

    pub fn growth() {
        
    }
}

struct Node {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    max_force: f32,
    max_speed: f32,
}

impl Node {
    pub fn new(position: Vector2<f32>, max_speed: f32, max_force: f32) -> Node {
        Node {
            position,
            velocity: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0),
            max_speed,
            max_force,
        }
    }

    pub fn apply_force(&mut self, force: Vector2<f32>) {
        self.acceleration.add_assign(force);
    }

    pub fn update(&mut self) {
        self.velocity.add_assign(self.acceleration);
        self.velocity = self.velocity.cap_magnitude(self.max_speed);
        self.position.add_assign(self.velocity);
        self.acceleration.mul_assign(0.0);
    }

    pub fn seek(&mut self, target: Vector2<f32>) -> Vector2<f32> {
        let mut desired: Vector2<f32> = target.sub(self.position);
        desired.set_magnitude(self.max_speed);
        let steer: Vector2<f32> = desired.sub(self.velocity);
        return steer.cap_magnitude(self.max_force);
    }
}
