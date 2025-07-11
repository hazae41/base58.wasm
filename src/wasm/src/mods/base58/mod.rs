use wasm_bindgen::prelude::*;

use memory_wasm::Memory;

use crate::libs::jse::rjse;

#[wasm_bindgen]
pub fn base58_encode(bytes: &Memory) -> String {
    bs58::encode(&bytes.inner).into_string()
}

#[wasm_bindgen]
pub fn base58_decode(text: &str) -> Result<Memory, JsError> {
    rjse!(bs58::decode(text).into_vec().map(Memory::new))
}
