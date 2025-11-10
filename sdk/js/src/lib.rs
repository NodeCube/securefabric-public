// SPDX-FileCopyrightText: 2025 NodeCube d.o.o. and contributors
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "wasm")]
use securefabric_core::aead::{decrypt_chacha_with_aad, encrypt_chacha_with_aad};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn encrypt(key: &[u8], nonce: &[u8], aad: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, JsValue> {
    if key.len() != 32 {
        return Err(JsValue::from_str("key must be32 bytes"));
    }
    if nonce.len() != 12 {
        return Err(JsValue::from_str("nonce must be12 bytes"));
    }
    let mut k = [0u8; 32];
    k.copy_from_slice(&key[..32]);
    let mut n = [0u8; 12];
    n.copy_from_slice(&nonce[..12]);
    let ct = encrypt_chacha_with_aad(&k, &n, aad, plaintext)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(ct)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn decrypt(
    key: &[u8],
    nonce: &[u8],
    aad: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, JsValue> {
    if key.len() != 32 {
        return Err(JsValue::from_str("key must be32 bytes"));
    }
    if nonce.len() != 12 {
        return Err(JsValue::from_str("nonce must be12 bytes"));
    }
    let mut k = [0u8; 32];
    k.copy_from_slice(&key[..32]);
    let mut n = [0u8; 12];
    n.copy_from_slice(&nonce[..12]);
    let pt = decrypt_chacha_with_aad(&k, &n, aad, ciphertext)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(pt)
}

#[cfg(not(feature = "wasm"))]
// When wasm feature is disabled provide noop stubs to keep crate buildable in workspace
pub fn encrypt(
    _key: &[u8],
    _nonce: &[u8],
    _aad: &[u8],
    _plaintext: &[u8],
) -> Result<Vec<u8>, String> {
    Err("wasm feature not enabled".to_string())
}

#[cfg(not(feature = "wasm"))]
pub fn decrypt(
    _key: &[u8],
    _nonce: &[u8],
    _aad: &[u8],
    _ciphertext: &[u8],
) -> Result<Vec<u8>, String> {
    Err("wasm feature not enabled".to_string())
}
