mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

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
    set_panic_hook();
    let output = day01::run(input);
    serde_wasm_bindgen::to_value(&output).unwrap()
}

#[wasm_bindgen]
pub fn day02(input: &str) -> JsValue {
    set_panic_hook();
    let output = day02::run(input);
    serde_wasm_bindgen::to_value(&output).unwrap()
}

#[wasm_bindgen]
pub fn day03(input: &str) -> JsValue {
    set_panic_hook();
    let output = day03::run(input);
    serde_wasm_bindgen::to_value(&output).unwrap()
}

#[wasm_bindgen]
pub fn day04(input: &str) -> JsValue {
    set_panic_hook();
    let output = day04::run(input);
    serde_wasm_bindgen::to_value(&output).unwrap()
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InputDay05 {
    stacks: [Vec<char>; 9],
    instructions: String,
}

#[wasm_bindgen]
pub fn day05(input_raw: JsValue) -> JsValue {
    set_panic_hook();
    let input: InputDay05 = serde_wasm_bindgen::from_value(input_raw).unwrap();
    let output: Output = day05::run(input);
    serde_wasm_bindgen::to_value(&output).unwrap()
}

#[wasm_bindgen]
pub fn day06(input: &str) -> JsValue {
    set_panic_hook();
    let output: Output = day06::run(input);
    serde_wasm_bindgen::to_value(&output).unwrap()
}

#[wasm_bindgen]
pub fn day07(input: &str) -> JsValue {
    set_panic_hook();
    let output: Output = day07::run(input);
    serde_wasm_bindgen::to_value(&output).unwrap()
}

#[wasm_bindgen]
pub fn day08(input: &str) -> JsValue {
    set_panic_hook();
    let output: Output = day08::run(input);
    serde_wasm_bindgen::to_value(&output).unwrap()
}

#[wasm_bindgen]
pub fn day09(input: &str) -> JsValue {
    set_panic_hook();
    let output: Output = day09::run(input);
    serde_wasm_bindgen::to_value(&output).unwrap()
}
