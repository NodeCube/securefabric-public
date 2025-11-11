// SPDX-License-Identifier: Apache-2.0
// Template for Rust SDK conformance tests
// Copy to sdk/rust/tests/conformance_tests.rs and implement

use securefabric_sdk::crypto::{encrypt, decrypt, sign, verify};
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct TestVectors {
    encryption: EncryptionVectors,
    signatures: SignatureVectors,
    replay_protection: ReplayProtectionVectors,
    tamper_detection: TamperDetectionVectors,
}

#[derive(Deserialize)]
struct EncryptionVectors {
    xchacha20_poly1305: Vec<EncryptionTest>,
}

#[derive(Deserialize)]
struct EncryptionTest {
    description: String,
    key: String,
    nonce: String,
    plaintext: String,
    aad: String,
    ciphertext: String,
    tag: String,
}

#[derive(Deserialize)]
struct SignatureVectors {
    ed25519: Vec<SignatureTest>,
}

#[derive(Deserialize)]
struct SignatureTest {
    description: String,
    secret_key: String,
    public_key: String,
    message: String,
    signature: String,
}

#[derive(Deserialize)]
struct ReplayProtectionVectors {
    description: String,
    tests: Vec<ReplayTest>,
}

#[derive(Deserialize)]
struct ReplayTest {
    description: String,
    #[serde(default)]
    window_size: Option<u64>,
    counters: Vec<u64>,
    expected: Vec<bool>,
}

#[derive(Deserialize)]
struct TamperDetectionVectors {
    description: String,
    tests: Vec<TamperTest>,
}

#[derive(Deserialize)]
struct TamperTest {
    description: String,
    key: String,
    nonce: String,
    #[serde(default)]
    ciphertext: Option<String>,
    #[serde(default)]
    original_ciphertext: Option<String>,
    #[serde(default)]
    tampered_ciphertext: Option<String>,
    #[serde(default)]
    tag: Option<String>,
    #[serde(default)]
    original_tag: Option<String>,
    #[serde(default)]
    tampered_tag: Option<String>,
    should_fail: bool,
}

fn hex_decode(s: &str) -> Vec<u8> {
    hex::decode(s).expect("Invalid hex string")
}

#[test]
fn test_encryption_conformance() {
    let vectors_json = fs::read_to_string("../tests/test_vectors.json")
        .expect("Failed to read test vectors");
    let vectors: TestVectors = serde_json::from_str(&vectors_json)
        .expect("Failed to parse test vectors");

    for test in vectors.encryption.xchacha20_poly1305 {
        println!("Testing: {}", test.description);

        let key = hex_decode(&test.key);
        let nonce = hex_decode(&test.nonce);
        let plaintext = hex_decode(&test.plaintext);
        let aad = hex_decode(&test.aad);
        let expected_ciphertext = hex_decode(&test.ciphertext);
        let expected_tag = hex_decode(&test.tag);

        // Test encryption
        let (ciphertext, tag) = encrypt(&key, &nonce, &plaintext, &aad)
            .expect("Encryption failed");

        assert_eq!(
            ciphertext, expected_ciphertext,
            "Ciphertext mismatch for: {}",
            test.description
        );
        assert_eq!(
            tag, expected_tag,
            "Tag mismatch for: {}",
            test.description
        );

        // Test decryption round-trip
        let decrypted = decrypt(&key, &nonce, &ciphertext, &aad, &tag)
            .expect("Decryption failed");

        assert_eq!(
            decrypted, plaintext,
            "Plaintext mismatch after round-trip: {}",
            test.description
        );
    }
}

#[test]
fn test_signature_conformance() {
    let vectors_json = fs::read_to_string("../tests/test_vectors.json")
        .expect("Failed to read test vectors");
    let vectors: TestVectors = serde_json::from_str(&vectors_json)
        .expect("Failed to parse test vectors");

    for test in vectors.signatures.ed25519 {
        println!("Testing: {}", test.description);

        let secret_key = hex_decode(&test.secret_key);
        let public_key = hex_decode(&test.public_key);
        let message = hex_decode(&test.message);
        let expected_signature = hex_decode(&test.signature);

        // Test signature generation
        let signature = sign(&secret_key, &message)
            .expect("Signature generation failed");

        assert_eq!(
            signature, expected_signature,
            "Signature mismatch for: {}",
            test.description
        );

        // Test signature verification
        let valid = verify(&public_key, &message, &signature)
            .expect("Signature verification failed");

        assert!(
            valid,
            "Signature verification failed for: {}",
            test.description
        );
    }
}

#[test]
fn test_replay_protection_conformance() {
    let vectors_json = fs::read_to_string("../tests/test_vectors.json")
        .expect("Failed to read test vectors");
    let vectors: TestVectors = serde_json::from_str(&vectors_json)
        .expect("Failed to parse test vectors");

    for test in vectors.replay_protection.tests {
        println!("Testing: {}", test.description);

        // Implement replay protection logic here
        // This will depend on your SDK's replay protection implementation

        // Example pseudocode:
        // let mut replay_filter = ReplayFilter::new(test.window_size);
        // for (counter, expected) in test.counters.iter().zip(test.expected.iter()) {
        //     let result = replay_filter.check(*counter);
        //     assert_eq!(result, *expected, "Replay check mismatch");
        // }
    }
}

#[test]
fn test_tamper_detection_conformance() {
    let vectors_json = fs::read_to_string("../tests/test_vectors.json")
        .expect("Failed to read test vectors");
    let vectors: TestVectors = serde_json::from_str(&vectors_json)
        .expect("Failed to parse test vectors");

    for test in vectors.tamper_detection.tests {
        println!("Testing: {}", test.description);

        let key = hex_decode(&test.key);
        let nonce = hex_decode(&test.nonce);

        if test.should_fail {
            // Test should detect tampering
            let ciphertext = if let Some(ct) = test.tampered_ciphertext {
                hex_decode(&ct)
            } else {
                hex_decode(test.ciphertext.as_ref().unwrap())
            };

            let tag = if let Some(t) = test.tampered_tag {
                hex_decode(&t)
            } else {
                hex_decode(test.tag.as_ref().unwrap())
            };

            let result = decrypt(&key, &nonce, &ciphertext, &[], &tag);
            assert!(
                result.is_err(),
                "Tamper detection failed for: {}",
                test.description
            );
        }
    }
}
