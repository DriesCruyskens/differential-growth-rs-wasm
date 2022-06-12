mod utils;

use std::f64::consts::PI;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
extern crate web_sys;

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
pub fn generate_points_of_circle_as_array(
    origin_x: f64,
    origin_y: f64,
    amount_of_points: usize,
    radius: f64,
) -> Box<[u32]> {
    set_panic_hook();
    let mut points: Vec<u32> = Vec::new();

    let h = origin_x;
    let k = origin_y;

    let two_pi = (2.0 * PI).round();

    let lin_space = iter_num_tools::lin_space(0.0..=two_pi, amount_of_points);

    log!("{:?}", lin_space);

    for theta in lin_space {
        let x = h + radius * f64::cos(theta as f64);
        let y = k + radius * f64::sin(theta as f64);

        points.push(x as u32);
        points.push(y as u32);
    }

    return points.into_boxed_slice();
}
