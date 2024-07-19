use wasm_bindgen::prelude::*;
use crate::tokenizer::Tokenizer;


#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    Ok(())
}


// #[wasm_bindgen]
// pub fn convert_to_syl(s: String) -> String {
//     crate::to_syl(s.to_lowercase().as_str())
// }

// #[wasm_bindgen]
// pub fn convert_to_lat(s: String) -> String {
//     crate::to_lat(s.as_str())
// }

#[wasm_bindgen]
pub fn tokenize(s: String) -> String {
    let tokens = Tokenizer::new(&s);
    let mut v = Vec::new();

    for t in tokens.collect::<Vec<_>>() {
        v.push(t.as_html());
    }
    v.join("")
}