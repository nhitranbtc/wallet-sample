use secure_storage::{SecureStorageError, Vault};
use zeroize::Zeroize;

#[test]
fn vault_round_trip_decrypts_under_correct_key() {
    let key = [7u8; 32];
    let seed = [42u8; 64];
    let mut ciphertext = seed.to_vec();
    let nonce = [1u8; 12];
    let vault = Vault::encrypt(&key, &nonce, &ciphertext).unwrap();
    ciphertext.zeroize();
    let decrypted = vault.decrypt(&key).unwrap();
    assert_eq!(decrypted.as_slice(), &seed[..]);
}

#[test]
fn vault_tamper_detection_fails() {
    let key = [9u8; 32];
    let mut contents = vec![1u8; 32];
    let nonce = [2u8; 12];
    let mut vault = Vault::encrypt(&key, &nonce, &contents).unwrap();
    contents.zeroize();
    vault.corrupt_for_test(0);
    let err = vault.decrypt(&key).unwrap_err();
    assert!(matches!(
        err,
        secure_storage::error::SecureStorageError::Integrity
    ));
}

#[test]
fn vault_supports_versioned_metadata() {
    let key = [3u8; 32];
    let nonce = [0u8; 12];
    let vault = Vault::encrypt_with_version(&key, &nonce, b"hello", 1).unwrap();
    assert_eq!(vault.version(), 1);
}

#[test]
fn seal_open_roundtrip() {
    let dek = [1u8; 32];
    let plaintext = b"hello world";
    let blob = Vault::seal(&dek, plaintext).unwrap();
    let opened = Vault::open(&dek, &blob).unwrap();
    assert_eq!(opened.as_slice(), plaintext);
}

#[test]
fn seal_produces_distinct_nonces() {
    let dek = [2u8; 32];
    let blob1 = Vault::seal(&dek, b"x").unwrap();
    let blob2 = Vault::seal(&dek, b"x").unwrap();
    assert_ne!(blob1, blob2);
}

#[test]
fn open_with_wrong_dek_returns_integrity() {
    let dek = [3u8; 32];
    let blob = Vault::seal(&dek, b"x").unwrap();
    let wrong = [4u8; 32];
    let err = Vault::open(&wrong, &blob).unwrap_err();
    assert_eq!(err, SecureStorageError::Integrity);
}

#[test]
fn open_with_truncated_blob_returns_integrity() {
    let dek = [5u8; 32];
    let blob = Vault::seal(&dek, b"x").unwrap();
    let err = Vault::open(&dek, &blob[..5]).unwrap_err();
    assert_eq!(err, SecureStorageError::Integrity);
}
