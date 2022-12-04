mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod day01;
mod day02;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Output {
    pub part1: String,
    pub part2: String,
}

#[wasm_bindgen]
pub fn day01(input: &str) -> JsValue {
    let output = day01::run(input);
    serde_wasm_bindgen::to_value(&output).unwrap()
}

#[wasm_bindgen]
pub fn day02(input: &str) -> JsValue {
    let output = day02::run(input);
    serde_wasm_bindgen::to_value(&output).unwrap()
}
