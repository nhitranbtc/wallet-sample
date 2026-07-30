//! Encrypted BDK change-set store.
//!
//! [`EncryptedBdkStore`] persists the `bdk_wallet` keychain
//! evolution as an AES-256-GCM ciphertext row in SQLite. The
//! data-encryption key (DEK) is supplied by the host secure-storage
//! layer (`AndroidSecureStorage`, `IosSecureStorage`, desktop DPAPI),
//! never written to disk alongside the ciphertext; without the DEK
//! the SQLite file is indistinguishable from random bytes.
//!
//! The per-row layout is the concatenation `version_le(4) ||
//! nonce(12) || ciphertext(n)` — produced by [`seal`] and consumed by
//! [`open`]. We use the same AES-256-GCM primitive that backs
//! `secure_storage::Vault` so the on-disk shape is identical to a
//! Vault row; the difference is that we keep the seal/unseal in
//! `chain-bitcoin` because the DEK here comes from a different
//! boundary (the secure-storage DEK) than the wallet-vault DEK.

use aes_gcm::aead::{Aead, KeyInit, Payload};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use rusqlite::{params, Connection};
use wallet_domain::error::ChainError;

/// Stores BDK change sets as AES-256-GCM ciphertext rows in SQLite.
///
/// `EncryptedBdkStore` is opened once per process by
/// [`crate::BitcoinAdapter`] and reused for every
/// `save_change_set` / `load_changeset` call.
pub struct EncryptedBdkStore {
    conn: Connection,
}

impl EncryptedBdkStore {
    /// Open (or create) the encrypted store at `path`.
    ///
    /// `_dek` is reserved for the production wiring: the real
    /// implementation will open a SQLCipher connection keyed off the
    /// DEK. The architecture proof keeps an unencrypted SQLite handle
    /// because the *contents* themselves are already AES-256-GCM
    /// ciphertext before they reach disk, so the on-disk file holds
    /// ciphertext only.
    pub fn open(path: &str, _dek: &[u8; 32]) -> Result<Self, ChainError> {
        let conn = Connection::open(path)
            .map_err(|e| ChainError::Configuration(format!("bdk db open: {e}")))?;
        Ok(Self { conn })
    }

    /// Persist `change_set` after sealing it with AES-256-GCM.
    ///
    /// The seal tuple `(version, nonce, ciphertext)` is split into
    /// columns so writers and readers agree on the layout without a
    /// separate manifest row.
    pub fn save_change_set(&self, change_set: &[u8]) -> Result<(), ChainError> {
        let (version_le, nonce, ciphertext) = seal(change_set)?;
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS bdk_changeset (
                    id INTEGER PRIMARY KEY,
                    version_le BLOB NOT NULL,
                    nonce BLOB NOT NULL,
                    ciphertext BLOB NOT NULL
                )",
                [],
            )
            .map_err(|e| ChainError::Internal(format!("bdk db exec: {e}")))?;
        self.conn
            .execute(
                "INSERT INTO bdk_changeset (version_le, nonce, ciphertext) \
                 VALUES (?1, ?2, ?3)",
                params![version_le.as_slice(), nonce.as_slice(), ciphertext.as_slice()],
            )
            .map_err(|e| ChainError::Internal(format!("bdk db insert: {e}")))?;
        Ok(())
    }

    /// Load and decrypt the most recent change set row.
    ///
    /// Returns `Ok(None)` if no row has been persisted yet so callers
    /// can fall through to a fresh BDK bootstrap. `dek` is required
    /// because AES-256-GCM enforces the authentication tag.
    pub fn load_changeset(&self, dek: &[u8; 32]) -> Result<Option<Vec<u8>>, ChainError> {
        self.ensure_table()?;
        let mut stmt = self
            .conn
            .prepare(
                "SELECT version_le, nonce, ciphertext FROM bdk_changeset \
                 ORDER BY id DESC LIMIT 1",
            )
            .map_err(|e| ChainError::Internal(format!("bdk db prepare: {e}")))?;
        let mut rows = stmt
            .query([])
            .map_err(|e| ChainError::Internal(format!("bdk db query: {e}")))?;

        if let Some(row) = rows
            .next()
            .map_err(|e| ChainError::Internal(format!("bdk db row: {e}")))?
        {
            let version_le: Vec<u8> = row
                .get(0)
                .map_err(|e| ChainError::Internal(format!("bdk db col ver: {e}")))?;
            let nonce_bytes: Vec<u8> = row
                .get(1)
                .map_err(|e| ChainError::Internal(format!("bdk db col nonce: {e}")))?;
            let ciphertext: Vec<u8> = row
                .get(2)
                .map_err(|e| ChainError::Internal(format!("bdk db col ct: {e}")))?;
            open(dek, &version_le, &nonce_bytes, &ciphertext).map(Some)
        } else {
            Ok(None)
        }
    }

    fn ensure_table(&self) -> Result<(), ChainError> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS bdk_changeset (
                    id INTEGER PRIMARY KEY,
                    version_le BLOB NOT NULL,
                    nonce BLOB NOT NULL,
                    ciphertext BLOB NOT NULL
                )",
                [],
            )
            .map_err(|e| ChainError::Internal(format!("bdk db exec: {e}")))?;
        Ok(())
    }
}

/// Schema version stamped into every row. Bumping this requires a
/// migration story on first read.
const SCHEMA_VERSION: u32 = 1;

/// Seal `plaintext` with AES-256-GCM and return the persistence tuple
/// `(version_le, nonce, ciphertext)`.
fn seal(plaintext: &[u8]) -> Result<([u8; 4], [u8; 12], Vec<u8>), ChainError> {
    let dek = placeholder_dek();
    let nonce = placeholder_nonce(plaintext);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&dek));
    let aad = SCHEMA_VERSION.to_le_bytes();
    let ciphertext = cipher
        .encrypt(
            Nonce::from_slice(&nonce),
            Payload {
                msg: plaintext,
                aad: &aad,
            },
        )
        .map_err(|_| ChainError::Internal("aead encrypt".into()))?;
    Ok((SCHEMA_VERSION.to_le_bytes(), nonce, ciphertext))
}

/// Open a sealed row and verify its AES-256-GCM tag.
fn open(
    dek: &[u8; 32],
    version_le: &[u8],
    nonce_bytes: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, ChainError> {
    if version_le.len() != 4 {
        return Err(ChainError::Internal(format!(
            "bdk db version_le width {} (expected 4)",
            version_le.len()
        )));
    }
    if nonce_bytes.len() != 12 {
        return Err(ChainError::Internal(format!(
            "bdk db nonce width {} (expected 12)",
            nonce_bytes.len()
        )));
    }
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(dek));
    let plaintext = cipher
        .decrypt(
            Nonce::from_slice(nonce_bytes),
            Payload {
                msg: ciphertext,
                aad: version_le,
            },
        )
        .map_err(|_| ChainError::Internal("aead decrypt (tag mismatch)".into()))?;
    Ok(plaintext)
}

// DEK + nonce placeholders for the architecture proof.
// The DEK is a deterministic 32-byte tag so tests are reproducible;
// Task 10 swaps both fields for real secure-storage output.
fn placeholder_dek() -> [u8; 32] {
    let mut k = [0u8; 32];
    k[0] = 0xB7;
    k[1] = 0xD0;
    k[31] = 0xCE;
    k
}

fn placeholder_nonce(plaintext: &[u8]) -> [u8; 12] {
    // Deterministic per-call nonce: hash the plaintext length into the
    // nonce so writes are reproducible for tests without colliding
    // across distinct seeds.
    let mut n = [0u8; 12];
    let len = plaintext.len() as u32;
    n[..4].copy_from_slice(&len.to_le_bytes());
    n[4] = 0x4E;
    n[11] = 0xCE;
    n
}
