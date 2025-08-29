
// WASM bindings for the ZK-STARK verifier using the ZisK library
use wasm_bindgen::prelude::*;
use proofman_verifier::verify;
use bytemuck::cast_slice;

// Set up panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn verify_stark(
    proof_bytes: &[u8],
    vk_bytes: &[u8],
) -> Result<bool, JsValue> {
    
    let proof: &[u64] = cast_slice(&proof_bytes);
    let vk: &[u64] = cast_slice(&vk_bytes);

    let result = verify(proof, vk);
    println!("Verification result: {:?}", result);
    Ok(result)
}
