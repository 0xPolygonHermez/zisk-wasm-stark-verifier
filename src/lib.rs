//! WASM bindings for the ZisK ZK-STARK verifier.
//!
//! Input is a proof as `cargo-zisk prove` saves it, ZisK's flat layout (what EthProofs
//! base64-encodes), which `zisk_verifier::decode_saved` reads:
//!
//! ```text
//! [minimal(1)][n_publics(1)][flag?|program_vk(4)|inputs(64)][proof(..)][vk(4)][hash_tag(1)]
//! ```

use wasm_bindgen::prelude::*;
use zisk_verifier::{decode_saved, decode_vk, verify};

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// Verify a ZisK vadcop_final proof against `vk_bytes`, the verification key from
/// the trusted setup (`vadcop_final_compressed.verkey.bin` for a minimal proof,
/// otherwise `vadcop_final.verkey.bin`).
///
/// The key the prover appends to the proof is discarded: a proof checked against
/// the key it ships with proves only that it is internally consistent, so the key
/// has to be pinned by the caller. The trailing hash tag only selects which verifier
/// runs: a wrong tag fails against `vk_bytes`.
#[wasm_bindgen]
pub fn verify_stark(proof_bytes: &[u8], vk_bytes: &[u8]) -> Result<bool, JsValue> {
    let js = |e: zisk_verifier::DecodeError| JsValue::from_str(&e.to_string());
    let vk = decode_vk(vk_bytes).map_err(js)?;
    let saved = decode_saved(proof_bytes).map_err(js)?;

    Ok(verify(&saved.proof, &vk).is_ok())
}
