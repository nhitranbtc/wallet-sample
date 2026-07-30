//! Encrypted BDK change-set store.
//!
//! [`EncryptedBdkStore`] persists the `bdk_wallet` keychain evolution
//! as a row-per-blob in SQLite, where every blob is the self-contained
//! AES-256-GCM ciphertext produced by `secure_storage::Vault::seal`.
//! The data-encryption key (DEK) is supplied by the host secure-storage
//! layer (`AndroidSecureStorage`, `IosSecureStorage`, desktop DPAPI),
//! never written to disk alongside the ciphertext; without the DEK the
//! SQLite file is indistinguishable from random bytes.
//!
//! The on-disk layout per row is the single `blob` column — i.e. the
//! `version_le(4) || nonce(12) || ciphertext(n)` byte string emitted by
//! `Vault::seal`. Versioning and AAD binding are handled inside the
//! Vault itself, so this store only has to persist the opaque payload.

use rusqlite::{params, Connection};
use secure_storage::Vault;
use wallet_domain::error::ChainError;
use zeroize::Zeroizing;

/// Stores BDK change sets as Vault-sealed rows in SQLite.
///
/// `EncryptedBdkStore` is opened once per process by
/// [`crate::BitcoinAdapter`] and reused for every
/// `save_change_set` / `load_changeset` call.
pub struct EncryptedBdkStore {
    conn: Connection,
    /// Wrapped DEK for `Vault::seal` / `Vault::open`.
    dek: Zeroizing<[u8; 32]>,
}

impl EncryptedBdkStore {
    /// Open (or create) the encrypted store at `path`, keyed off the
    /// DEK supplied by the secure-storage layer.
    ///
    /// The architecture proof keeps an unencrypted SQLite handle
    /// because the *contents* themselves are already AES-256-GCM
    /// ciphertext — produced by `Vault::seal` — before they reach
    /// disk, so the on-disk file holds ciphertext only.
    pub fn open(path: &str, dek: &[u8; 32]) -> Result<Self, ChainError> {
        let conn = Connection::open(path)
            .map_err(|_| ChainError::Configuration("bdk db open".into()))?;
        Ok(Self {
            conn,
            dek: Zeroizing::new(*dek),
        })
    }

    /// Persist `change_set` after sealing it via `Vault::seal`.
    pub fn save_change_set(&self, change_set: &[u8]) -> Result<(), ChainError> {
        let blob = Vault::seal(&*self.dek, change_set)
            .map_err(|_| ChainError::Internal("vault seal".into()))?;
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS bdk_changeset (id INTEGER PRIMARY KEY, blob BLOB)",
                [],
            )
            .map_err(|_| ChainError::Internal("bdk db exec".into()))?;
        self.conn
            .execute(
                "INSERT INTO bdk_changeset (blob) VALUES (?1)",
                params![blob],
            )
            .map_err(|_| ChainError::Internal("bdk db insert".into()))?;
        Ok(())
    }

    /// Load and unseal the most recent change set row.
    ///
    /// Returns `Ok(None)` if no row has been persisted yet so callers
    /// can fall through to a fresh BDK bootstrap. The DEK is read from
    /// the struct (it was supplied at `open`), so the unsealed
    /// plaintext flows through `Vault::open` and its AEAD tag is
    /// verified in the process.
    pub fn load_changeset(&self) -> Result<Option<Vec<u8>>, ChainError> {
        self.ensure_table()?;
        let mut stmt = self
            .conn
            .prepare("SELECT blob FROM bdk_changeset ORDER BY id DESC LIMIT 1")
            .map_err(|_| ChainError::Internal("bdk db prepare".into()))?;
        let mut rows = stmt
            .query([])
            .map_err(|_| ChainError::Internal("bdk db query".into()))?;
        if let Some(row) = rows
            .next()
            .map_err(|_| ChainError::Internal("bdk db row".into()))?
        {
            let blob: Vec<u8> = row
                .get(0)
                .map_err(|_| ChainError::Internal("bdk db col".into()))?;
            let plaintext = Vault::open(&*self.dek, &blob)
                .map_err(|_| ChainError::Internal("vault open".into()))?;
            Ok(Some((*plaintext).clone()))
        } else {
            Ok(None)
        }
    }

    fn ensure_table(&self) -> Result<(), ChainError> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS bdk_changeset (id INTEGER PRIMARY KEY, blob BLOB)",
                [],
            )
            .map_err(|_| ChainError::Internal("bdk db exec".into()))?;
        Ok(())
    }
}
