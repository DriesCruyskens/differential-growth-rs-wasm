mod utils;

use differential_growth::DifferentialGrowth;
use nalgebra::Point2;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
}

/// Returns an array of numbers that can be converted to an array
/// of points. It's a flattened array that signifies x and y values. For ex:
///
/// point1.x = returned_array[0]
/// point1.y = returned_array[1]
/// point2.x = returned_array[2]
/// point2.y = returned_array[3]
/// ...
///
#[wasm_bindgen]
pub fn generate_points_on_circle(
    origin_x: f64,
    origin_y: f64,
    radius: f64,
    amount_of_points: usize,
) -> Box<[f64]> {
    let points = differential_growth::generate_points_on_circle(
        origin_x,
        origin_y,
        radius,
        amount_of_points,
    );

    // Converting points to array of numbers.
    let n = points.len() * 2;
    let mut export: Vec<f64> = Vec::with_capacity(n);

    for i in 0..points.len() {
        export.push(points[i].x);
        export.push(points[i].y);
    }

    export.into_boxed_slice()
}

#[wasm_bindgen]
pub struct DifferentialGrowthWasm {
    differential_growth: DifferentialGrowth,
}

#[wasm_bindgen]
impl DifferentialGrowthWasm {
    #[wasm_bindgen(constructor)]
    /// `starting_points` is an array of numbers that will be converted to an array
    /// of points. It's a flattened array that signifies x and y values. For ex:
    ///
    /// point1.x = starting_points[0]
    ///
    /// point1.y = starting_points[1]
    ///
    /// point2.x = starting_points[2]
    ///
    /// point2.y = starting_points[3]
    ///
    pub fn new(
        starting_points: &[f64],
        max_force: f64,
        max_speed: f64,
        desired_separation: f64,
        separation_cohesion_ratio: f64,
        max_edge_len: f64,
    ) -> DifferentialGrowthWasm {
        // Converting array of numbers to array of Points.
        let mut input_points = Vec::with_capacity(starting_points.len() / 2);
        for pair in starting_points.chunks(2) {
            let x: f64 = pair[0];
            let y: f64 = pair[1];

            input_points.push(Point2::new(x, y));
        }

        let differential_growth = DifferentialGrowth::new(
            input_points,
            max_force,
            max_speed,
            desired_separation,
            separation_cohesion_ratio,
            max_edge_len,
        );

        DifferentialGrowthWasm {
            differential_growth,
        }
    }

    /// Returns the amount of points.
    pub fn tick(&mut self) {
        self.differential_growth.tick();
    }

    /// Returns the amount of points or nodes in the algorithm.
    pub fn get_amount_of_points(&self) -> usize {
        self.differential_growth.nodes.len()
    }

    /// Rust analyzer suggests the canvas() function on the &CanvasRenderingContext2d object
    /// but then complains it doesn't exists after using it.
    /// Passing canvas width and height manually instead.
    pub fn render(&self, ctx: &CanvasRenderingContext2d, canvas_width: f64, canvas_height: f64) {
        ctx.clear_rect(0.0, 0.0, canvas_width, canvas_height);
        ctx.begin_path();
        ctx.move_to(
            self.differential_growth
                .nodes
                .first()
                .expect("no points")
                .position
                .x,
            self.differential_growth
                .nodes
                .first()
                .expect("no points")
                .position
                .y,
        );

        for node in self.differential_growth.nodes.iter().skip(1) {
            ctx.line_to(node.position.x, node.position.y);
        }

        ctx.close_path();
        ctx.stroke();
    }

    /// Exports the positions of the nodes as an array of numbers. This is necessary
    /// because wasm-bindgen does only supports passing basic types between javascript
    /// and wasm.
    // https://rustwasm.github.io/docs/wasm-bindgen/reference/types/boxed-number-slices.html
    pub fn export_as_slice(&self) -> Box<[f64]> {
        let n = self.differential_growth.nodes.len() * 2;
        let mut export: Vec<f64> = Vec::with_capacity(n);

        for i in 0..self.differential_growth.nodes.len() {
            export.push(self.differential_growth.nodes[i].position.x);
            export.push(self.differential_growth.nodes[i].position.y);
        }

        return export.into_boxed_slice();
    }
}
