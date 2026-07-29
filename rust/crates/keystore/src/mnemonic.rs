use bip39::{Language, Mnemonic as Bip39};
use wallet_domain::error::WalletError;
use zeroize::{Zeroize, ZeroizeOnDrop};

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
        Ok(Self {
            phrase: bip.to_string(),
            seed: bip.to_seed(""),
        })
    }

    pub fn from_phrase(phrase: &str, passphrase: &str) -> Result<Self, WalletError> {
        let bip =
            Bip39::parse_in(Language::English, phrase).map_err(|_| WalletError::InvalidMnemonic)?;
        Ok(Self {
            phrase: phrase.to_string(),
            seed: bip.to_seed(passphrase),
        })
    }

    pub fn seed(&self) -> &[u8; 64] {
        &self.seed
    }

    pub fn zeroize_phrase(&mut self) {
        self.phrase.zeroize();
    }

    #[cfg(test)]
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
