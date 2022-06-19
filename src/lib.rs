mod utils;

use std::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, MulAssign, Sub, SubAssign},
};

use nalgebra::{distance, Point2, Vector2};
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

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
}

#[wasm_bindgen]
pub struct Line {
    nodes: Vec<Node>,
    max_force: f32,
    max_speed: f32,
    desired_separation: f32,
    desired_separation_sq: f32,
    separation_cohesion_ration: f32,
    max_edge_length: f32,
}

#[wasm_bindgen]
impl Line {
    #[wasm_bindgen(constructor)]
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
        // Generate points on a circle and map to Nodes.
        let nodes: Vec<Node> =
            generate_points_of_circle(origin_x, origin_y, amount_of_points, radius)
                .into_iter()
                .map(|point: Point2<f32>| Node::new(point, max_speed, max_force))
                .collect();
        Line {
            nodes,
            max_force,
            max_speed,
            desired_separation,
            desired_separation_sq: desired_separation.powi(2),
            separation_cohesion_ration,
            max_edge_length: max_edge_len,
        }
    }

    pub fn run(&mut self) {
        self.differentiate();
        self.growth();
    }

    // https://rustwasm.github.io/docs/wasm-bindgen/reference/types/boxed-number-slices.html
    pub fn export_as_slice(&self) -> Box<[f32]> {
        let n = self.nodes.len() * 2;
        let mut export: Vec<f32> = Vec::with_capacity(n);

        for i in 0..self.nodes.len() {
            export.push(self.nodes[i].position.x);
            export.push(self.nodes[i].position.y);
        }

        return export.into_boxed_slice();
    }
}

// Having two different `impl Line` sections because wasm_bindgen attribute macro
// is not supporting Vectors from nalgebra
impl Line {
    pub fn add_node_at(&mut self, node: Node, index: usize) {
        self.nodes.insert(index, node);
    }

    pub fn growth(&mut self) {
        let mut new_nodes: Vec<(Node, usize)> = Vec::with_capacity(self.nodes.len() - 1);
        let mut amount_nodes_added = 0;

        for i in 0..self.nodes.len() - 1 {
            let n1: &Node = &self.nodes[i];
            let n2: &Node = &self.nodes[i + 1];

            let distance: f32 = distance(&n1.position, &n2.position);

            if distance > self.max_edge_length {
                // Inserting new nodes shifts the index of the original nodes.
                // To compensate we shift the index with it.
                let index: usize = i + 1 + amount_nodes_added;
                amount_nodes_added.add_assign(1);
                let middle_node: Vector2<f32> = n1.position.coords.add(n2.position.coords).div(2.0);
                new_nodes.push((
                    Node::new(
                        Point2::new(middle_node.x, middle_node.y),
                        self.max_speed,
                        self.max_force,
                    ),
                    index,
                ));
            }
        }

        for new_node in new_nodes {
            self.add_node_at(new_node.0, new_node.1);
        }
    }

    pub fn differentiate(&mut self) {
        let separation_forces: Vec<Vector2<f32>> = self.get_separation_forces();
        let cohesion_forces: Vec<Vector2<f32>> = self.get_edge_cohesion_forces();

        for i in 0..self.nodes.len() {
            let mut separation: Vector2<f32> = separation_forces[i];
            let cohesion: Vector2<f32> = cohesion_forces[i];

            separation.mul_assign(self.separation_cohesion_ration);

            self.nodes[i].apply_force(separation);
            self.nodes[i].apply_force(cohesion);
            self.nodes[i].update();
        }
    }

    pub fn get_separation_forces(&self) -> Vec<Vector2<f32>> {
        let n: usize = self.nodes.len();
        let mut separate_forces: Vec<Vector2<f32>> = vec![Vector2::default(); n];
        let mut near_nodes: Vec<i32> = vec![0; n];

        for i in 0..n {
            let nodei = &self.nodes[i];
            for j in 0..n {
                let nodej = &self.nodes[j];
                let force_ij: Vector2<f32> = self.get_separation_force(nodei, nodej);
                if force_ij.magnitude() > 0.0 {
                    separate_forces[i].add_assign(force_ij);
                    separate_forces[j].sub_assign(force_ij);
                    near_nodes[i].add_assign(1);
                    near_nodes[j].add_assign(1);
                }
            }

            if near_nodes[i] > 0 {
                separate_forces[i].div_assign(near_nodes[i] as f32);
            }

            if separate_forces[i].magnitude() > 0.0 {
                separate_forces[i].set_magnitude(self.max_speed);
                separate_forces[i].sub_assign(self.nodes[i].velocity);
                separate_forces[i] = separate_forces[i].cap_magnitude(self.max_force);
            }
        }

        return separate_forces;
    }

    pub fn get_separation_force(&self, n1: &Node, n2: &Node) -> Vector2<f32> {
        let mut steer: Vector2<f32> = Vector2::default();
        let distance: f32 = distance(&n1.position, &n2.position);

        if distance > 0.0 && distance < self.desired_separation {
            let mut diff: Vector2<f32> = n1.position.sub(n2.position);
            diff = diff.normalize();
            diff.div_assign(distance);
            steer.add_assign(diff);
        }

        // Optimised version by defering sqrt() to inside if statement.
        // let distance_sq: f32 = (n2.position.x - n1.position.x).powi(2) + (n2.position.y - n1.position.y).powi(2);

        // if distance_sq > 0.0 && distance_sq < self.desired_separation_sq {
        //     let mut diff: Vector2<f32> = n1.position.sub(n2.position);
        //     diff = diff.normalize();
        //     diff.div_assign(distance_sq.sqrt());
        //     steer.add_assign(diff);
        // }

        return steer;
    }

    pub fn get_edge_cohesion_forces(&self) -> Vec<Vector2<f32>> {
        let n: usize = self.nodes.len();
        let mut cohesion_forces: Vec<Vector2<f32>> = vec![Vector2::default(); n];

        for i in 0..n {
            let mut sum: Vector2<f32> = Vector2::default();
            if i != 0 && i != n - 1 {
                sum.add_assign(self.nodes[i - 1].position.coords);
                sum.add_assign(self.nodes[i + 1].position.coords);
            } else if i == 0 {
                sum.add_assign(self.nodes[n - 1].position.coords);
                sum.add_assign(self.nodes[i + 1].position.coords);
            } else if i == n - 1 {
                sum.add_assign(self.nodes[i - 1].position.coords);
                sum.add_assign(self.nodes[0].position.coords);
            }
            sum.div_assign(2.0);
            cohesion_forces[i] = self.nodes[i].seek(sum);
        }

        return cohesion_forces;
    }
}

pub struct Node {
    position: Point2<f32>,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    max_force: f32,
    max_speed: f32,
}

impl Node {
    pub fn new(position: Point2<f32>, max_speed: f32, max_force: f32) -> Node {
        Node {
            position,
            velocity: Vector2::default(),
            acceleration: Vector2::default(),
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

    pub fn seek(&self, target: Vector2<f32>) -> Vector2<f32> {
        let mut desired: Vector2<f32> = target.sub(self.position.coords);
        desired.set_magnitude(self.max_speed);
        let steer: Vector2<f32> = desired.sub(self.velocity);
        return steer.cap_magnitude(self.max_force);
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("pos", &self.position)
            .field("vel", &self.velocity)
            .field("acc", &self.acceleration)
            .finish()
    }
}
