use std::f64::consts::PI;

use iter_num_tools::{lin_space, LinSpace};
use micromath::vector;

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
    origin_x: f32,
    origin_y: f32,
    amount_of_points: usize,
    radius: f32,
) -> Vec<vector::F32x2> {
    let mut points: Vec<vector::F32x2> = Vec::new();

    let h: f32 = origin_x;
    let k: f32 = origin_y;

    let two_pi: f64 = (2.0 * PI).round();

    let lin_space: LinSpace<f32> = lin_space(0.0..=two_pi as f32, amount_of_points);

    for theta in lin_space {
        let x: f32 = h + radius * f32::cos(theta);
        let y: f32 = k + radius * f32::sin(theta);
        let point: vector::F32x2 = vector::F32x2 { x, y };
        points.push(point);
    }

    return points;
}
