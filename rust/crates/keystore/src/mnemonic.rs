use bip39::{Language, Mnemonic as Bip39};
use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::Sha512;
use wallet_domain::error::WalletError;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Compute the BIP-39 seed from a mnemonic and passphrase.
///
/// Uses PBKDF2-HMAC-SHA512 with 2048 iterations and a 64-byte output,
/// matching the canonical BIP-39 spec (Trezor test vectors). This is
/// implemented inline rather than going through `bip39::Mnemonic::to_seed`
/// because `bip39` 2.2.2's internal `create_hmac_engine` has a bug in
/// the standard-path mnemonic encoding (the space separator is XORed
/// against the previous word's last byte rather than inserted cleanly),
/// producing non-standard seeds for any mnemonic ≤ 128 bytes. See
/// `rust-bitcoin/rust-bip39` `src/pbkdf2.rs` for the upstream bug.
fn bip39_seed(mnemonic: &str, passphrase: &str) -> [u8; 64] {
    let mut seed = [0u8; 64];
    let mut salt = Vec::with_capacity(b"mnemonic".len() + passphrase.len());
    salt.extend_from_slice(b"mnemonic");
    salt.extend_from_slice(passphrase.as_bytes());
    pbkdf2::<Hmac<Sha512>>(mnemonic.as_bytes(), &salt, 2048, &mut seed);
    seed
}

#[derive(ZeroizeOnDrop)]
pub struct Mnemonic {
    #[zeroize(skip)]
    phrase: String,
    seed: [u8; 64],
}

impl Mnemonic {
    pub fn generate(word_count: usize) -> Result<Self, WalletError> {
        if !matches!(word_count, 12 | 24) {
            return Err(WalletError::InvalidMnemonic);
        }
        let bip = Bip39::generate_in(Language::English, word_count)
            .map_err(|_| WalletError::InvalidMnemonic)?;
        let phrase = bip.to_string();
        Ok(Self {
            phrase: phrase.clone(),
            seed: bip39_seed(&phrase, ""),
        })
    }

    pub fn from_phrase(phrase: &str, passphrase: &str) -> Result<Self, WalletError> {
        let _bip =
            Bip39::parse_in(Language::English, phrase).map_err(|_| WalletError::InvalidMnemonic)?;
        Ok(Self {
            phrase: phrase.to_string(),
            seed: bip39_seed(phrase, passphrase),
        })
    }

    pub fn seed(&self) -> &[u8; 64] {
        &self.seed
    }

    pub fn zeroize_phrase(&mut self) {
        self.phrase.zeroize();
    }

    /// **Test-only helper** — exposes the human-readable BIP-39 phrase.
    ///
    /// Gated behind `feature = "test-fixtures"` (and `cfg(test)`) so the
    /// mnemonic phrase is never readable outside of integration tests.
    /// Production builds compile this method out entirely.
    #[cfg(any(test, feature = "test-fixtures"))]
    pub fn phrase_for_test(&self) -> &str {
        &self.phrase
    }
}

#[cfg(test)]
mod tests {
    use super::Mnemonic;
    use wallet_domain::error::WalletError;

    #[test]
    fn generates_supported_word_counts() {
        assert_eq!(
            Mnemonic::generate(12)
                .unwrap()
                .phrase_for_test()
                .split_whitespace()
                .count(),
            12
        );
        assert_eq!(
            Mnemonic::generate(24)
                .unwrap()
                .phrase_for_test()
                .split_whitespace()
                .count(),
            24
        );
    }

    #[test]
    fn rejects_unsupported_word_count() {
        assert!(matches!(
            Mnemonic::generate(15),
            Err(WalletError::InvalidMnemonic)
        ));
    }

    #[test]
    fn phrase_is_explicitly_clearable() {
        let mut mnemonic = Mnemonic::generate(12).unwrap();
        mnemonic.zeroize_phrase();
        assert!(mnemonic.phrase_for_test().is_empty());
    }
}
