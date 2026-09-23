//! WASM bindings for the ZisK ZK-STARK verifier.
//!
//! Input is `Proof::get_proof_u64()` flattened little-endian — the bytes the prover
//! writes with `--proof.save` and base64-encodes for EthProofs:
//!
//! ```text
//! [minimal(1)][n_publics(1)][flag?|program_vk(4)|inputs(64)][proof(..)][vk(4)][hash_tag(1)]
//! ```
//!
//! `verify_vadcop_final_proof` validates that header itself, so all this module
//! does is split the trailing key and hash-family tag off the tail.

use wasm_bindgen::prelude::*;
use zisk_verifier::{
    hash_id_from_tag, verify_vadcop_final_proof, HASH_TAG_LEN_WORDS, VADCOP_VK_LEN_WORDS,
};

const VK_BYTES: usize = VADCOP_VK_LEN_WORDS * 8;

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
    if vk_bytes.len() != VK_BYTES {
        return Err(JsValue::from_str(&format!(
            "vk must be {VK_BYTES} bytes ({VADCOP_VK_LEN_WORDS} little-endian u64 words), got {}",
            vk_bytes.len()
        )));
    }
    if !proof_bytes.len().is_multiple_of(8) {
        return Err(JsValue::from_str(&format!(
            "proof length must be a multiple of 8, got {}",
            proof_bytes.len()
        )));
    }

    let mut proof = to_words(proof_bytes);
    let len = proof
        .len()
        .checked_sub(VADCOP_VK_LEN_WORDS + HASH_TAG_LEN_WORDS)
        .ok_or_else(|| JsValue::from_str("proof too short"))?;
    let tag = proof[proof.len() - 1];
    let hash = hash_id_from_tag(tag)
        .ok_or_else(|| JsValue::from_str(&format!("unrecognized hash tag {tag}")))?;
    proof.truncate(len);

    Ok(verify_vadcop_final_proof(&proof, &to_words(vk_bytes), hash))
}

fn to_words(bytes: &[u8]) -> Vec<u64> {
    bytes
        .chunks_exact(8)
        .map(|c| u64::from_le_bytes(c.try_into().expect("chunks_exact(8) yields 8 bytes")))
        .collect()
}
