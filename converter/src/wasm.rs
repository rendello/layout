use wasm_bindgen::prelude::*;


// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    Ok(())
}


#[wasm_bindgen]
pub fn convert_to_syl(s: String) -> String {
    crate::to_syl(s.to_lowercase().as_str())
}