use crate::WalletSession;
use bech32::{ToBase32, Variant};
use ed25519_dalek::SigningKey as Ed25519SigningKey;
use k256::ecdsa::SigningKey as K256SigningKey;
use ripemd::Ripemd160;
use secp256k1::{Secp256k1, SecretKey};
use sha2::{Digest, Sha256};
use sha3::Keccak256;
use wallet_domain::error::WalletError;

pub trait Derive {
    fn derive_evm_address(&self) -> Result<String, WalletError>;
    fn derive_bitcoin_address(&self, change: bool) -> Result<String, WalletError>;
    fn derive_solana_address(&self) -> Result<String, WalletError>;
    fn derive_tron_address(&self) -> Result<String, WalletError>;
    fn derive_evm_key(&self) -> Result<SecretKey, WalletError>;
    fn derive_bitcoin_key(&self) -> Result<SecretKey, WalletError>;
    fn derive_solana_key(&self) -> Result<Ed25519SigningKey, WalletError>;
    fn derive_tron_key(&self) -> Result<K256SigningKey, WalletError>;
}

impl Derive for WalletSession {
    fn derive_evm_address(&self) -> Result<String, WalletError> {
        let public = self
            .derive_evm_key()?
            .public_key(&Secp256k1::new())
            .serialize_uncompressed();
        let hash = Keccak256::digest(&public[1..]);
        Ok(format!("0x{}", hex::encode(&hash[12..])))
    }

    fn derive_bitcoin_address(&self, change: bool) -> Result<String, WalletError> {
        let path = format!("m/84'/1'/0'/{}", usize::from(change));
        let key = derive_secp256k1(&self.seed, &path)?;
        // HASH160 = RIPEMD-160(SHA-256(public_key)) — the standard fingerprint
        // used by both P2PKH and P2WPKH Bitcoin addresses. SHA-256 alone is
        // not the same hash and would produce invalid bech32 addresses.
        let sha_hash = Sha256::digest(key.public_key(&Secp256k1::new()).serialize());
        let witness_program = Ripemd160::digest(sha_hash);
        let mut data = vec![0x06, 0x14];
        data.extend_from_slice(&witness_program);
        bech32::encode("tb", data.to_base32(), Variant::Bech32)
            .map_err(|_| WalletError::DerivationFailed)
    }

    fn derive_solana_address(&self) -> Result<String, WalletError> {
        Ok(bs58::encode(self.derive_solana_key()?.verifying_key().to_bytes()).into_string())
    }

    fn derive_tron_address(&self) -> Result<String, WalletError> {
        let point = self
            .derive_tron_key()?
            .verifying_key()
            .to_encoded_point(false);
        let hash = Keccak256::digest(&point.as_bytes()[1..]);
        let mut payload = Vec::with_capacity(25);
        payload.push(0x41);
        payload.extend_from_slice(&hash[12..]);
        payload.extend_from_slice(&double_sha256(&payload)[..4]);
        Ok(bs58::encode(payload).into_string())
    }

    fn derive_evm_key(&self) -> Result<SecretKey, WalletError> {
        derive_secp256k1(&self.seed, "m/44'/60'/0'/0/0")
    }

    fn derive_bitcoin_key(&self) -> Result<SecretKey, WalletError> {
        derive_secp256k1(&self.seed, "m/84'/1'/0'/0")
    }

    fn derive_solana_key(&self) -> Result<Ed25519SigningKey, WalletError> {
        let secret = derive_secret(&self.seed, "m/44'/501'/0'/0/0")?;
        Ok(Ed25519SigningKey::from_bytes(&secret))
    }

    fn derive_tron_key(&self) -> Result<K256SigningKey, WalletError> {
        let secret = derive_secret(&self.seed, "m/44'/195'/0'/0/0")?;
        K256SigningKey::from_bytes((&secret).into()).map_err(|_| WalletError::DerivationFailed)
    }
}

fn derive_secret(seed: &[u8; 64], path: &str) -> Result<[u8; 32], WalletError> {
    let extended = tiny_hderive::bip32::ExtendedPrivKey::derive(seed, path)
        .map_err(|_| WalletError::DerivationFailed)?;
    Ok(extended.secret())
}

fn derive_secp256k1(seed: &[u8; 64], path: &str) -> Result<SecretKey, WalletError> {
    SecretKey::from_slice(&derive_secret(seed, path)?).map_err(|_| WalletError::DerivationFailed)
}

fn double_sha256(data: &[u8]) -> [u8; 32] {
    Sha256::digest(Sha256::digest(data)).into()
}
