use secure_storage::Vault;
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
