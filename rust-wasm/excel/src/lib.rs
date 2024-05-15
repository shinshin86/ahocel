use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello from Rust!".into()
}

#[wasm_bindgen]
pub fn sum(numbers: &[f64]) -> f64 {
    numbers.iter().sum()
}