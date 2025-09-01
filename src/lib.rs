// WASM bindings for the ZK-STARK verifier using the ZisK library
use proofman_verifier::verify;
use wasm_bindgen::prelude::*;

// Set up panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn verify_stark(proof_bytes: &[u8], vk_bytes: &[u8]) -> Result<bool, JsValue> {
    let result = verify(proof_bytes, vk_bytes);
    println!("Verification result: {:?}", result);
    Ok(result)
}
