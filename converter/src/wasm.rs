use wasm_bindgen::prelude::*;


#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    Ok(())
}


#[wasm_bindgen]
pub fn convert_to_syl(s: String) -> String {
    crate::to_syl(s.to_lowercase().as_str())
}

#[wasm_bindgen]
pub fn convert_to_lat(s: String) -> String {
    crate::to_lat(s.as_str())
}