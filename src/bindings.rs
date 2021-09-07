use wasm_bindgen::prelude::*;

// wasm-bindgen will automatically take care of including this script
#[wasm_bindgen(module = "/src/play.js")]
extern "C" {
    #[wasm_bindgen(js_name = "play")]
    pub fn play() -> bool;
}
