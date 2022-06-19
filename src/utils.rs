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
    origin_x: f64,
    origin_y: f64,
    amount_of_points: usize,
    radius: f64,
) -> Vec<Point2<f64>> {
    let mut points: Vec<Point2<f64>> = Vec::new();

    let h: f64 = origin_x;
    let k: f64 = origin_y;

    let two_pi = 2.0 * PI as f64;
    let step = two_pi / amount_of_points as f64;
    let mut theta: f64 = 0.0;

    while theta < two_pi {
        let x: f64 = h + radius * f64::cos(theta);
        let y: f64 = k + radius * f64::sin(theta);
        points.push(Point2::new(x, y));
        theta.add_assign(step);
    }

    return points;
}
