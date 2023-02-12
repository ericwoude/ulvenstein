use wasm_bindgen::prelude::*;

mod ulvenstein;

#[wasm_bindgen]
pub fn request_screen() -> Result<JsValue, JsValue> {
    let rectangles = ulvenstein::game_loop();
    Ok(serde_wasm_bindgen::to_value(&rectangles)?)
}

#[wasm_bindgen]
pub fn register_input(val: JsValue) -> Result<(), JsValue> {
    let keys: ulvenstein::Keys = serde_wasm_bindgen::from_value(val)?;
    ulvenstein::INPUT.write().unwrap().push(keys);
    Ok(())
}
