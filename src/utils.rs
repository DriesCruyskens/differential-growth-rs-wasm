use std::f64::consts::PI;

use iter_num_tools::{lin_space, LinSpace};
use micromath::{vector::Vector2d, F32};

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn generate_points_of_circle(
    origin_x: F32,
    origin_y: F32,
    amount_of_points: usize,
    radius: F32,
) -> Vec<Vector2d<F32>> {
    let mut points: Vec<Vector2d<F32>> = Vec::new();

    let h: F32 = origin_x;
    let k: F32 = origin_y;

    let two_pi: f32 = (2.0 * PI as f32).round();

    let lin_space: LinSpace<f32> = lin_space(0.0..=two_pi, amount_of_points);

    for i in lin_space {
        let theta: F32 = F32 { 0: i };
        let x: F32 = h + radius * F32::cos(theta);
        let y: F32 = k + radius * F32::sin(theta);
        let point: Vector2d<F32> = Vector2d { x, y };
        points.push(point);
    }

    return points;
}
