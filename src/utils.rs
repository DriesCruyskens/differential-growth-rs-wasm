use std::f64::consts::PI;

use nalgebra::Point2;

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
    origin_x: f64,
    origin_y: f64,
    amount_of_points: f64,
    radius: f64,
) -> Vec<Point2<f64>> {
    let mut points: Vec<Point2<f64>> = Vec::new();

    let h = origin_x;
    let k = origin_y;

    let two_pi = (2.0 * PI).round() as usize;
    let step = (2.0 * PI / amount_of_points).round() as usize;

    for theta in (0..two_pi).step_by(step) {
        let x = h + radius * f64::cos(theta as f64);
        let y = k + radius * f64::sin(theta as f64);
        let point: Point2<f64> = Point2::new(x, y);
        points.push(point);
    }

    return points;
}
