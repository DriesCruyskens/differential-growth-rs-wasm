use std::{f64::consts::PI, ops::AddAssign};

use nalgebra::Point2;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

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

// https://www.mathopenref.com/coordcirclealgorithm.html
pub fn generate_points_of_circle(
    origin_x: f32,
    origin_y: f32,
    amount_of_points: usize,
    radius: f32,
) -> Vec<Point2<f32>> {
    let mut points: Vec<Point2<f32>> = Vec::new();

    let h: f32 = origin_x;
    let k: f32 = origin_y;

    let two_pi = 2.0 * PI as f32;
    let step = two_pi / amount_of_points as f32;
    let mut theta: f32 = 0.0;

    while theta < two_pi {
        let x: f32 = h + radius * f32::cos(theta);
        let y: f32 = k + radius * f32::sin(theta);
        points.push(Point2::new(x, y));
        theta.add_assign(step);
    }

    return points;
}
