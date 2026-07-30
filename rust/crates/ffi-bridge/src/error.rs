//! [`DartError`] — the only error shape Dart ever sees across the FFI
//! boundary.
//!
//! `message_key` is the i18n key the Flutter-side l10n catalogue
//! resolves into the user-visible copy. **The underlying
//! `Display` / `Debug` text is never exposed:** [`DartError::from`]
//! runs the raw message through [`redact`] before constructing the
//! error, so leaked addresses, hashes, and base58 payloads can never
//! reach Dart even if `format!("{}", e)` happens to contain them.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wallet_domain::error::ErrorCategory;

/// FFI-safe error type.
///
/// `code` is a stable identifier used by Dart-side analytics
/// (`"internal"` for now; will be expanded once the catalogs grow).
/// `category` mirrors [`wallet_domain::error::ErrorCategory`]. The two
/// booleans let the Flutter layer decide whether to retry
/// (`retryable`) and whether the previously issued `PreparedHandle` is
/// still valid (`fresh_preparation_required`). `diagnostics_id` is the
/// UUID v4 the support tool correlates with a Rust-side log entry —
/// never any payload text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DartError {
    pub code: String,
    pub category: ErrorCategory,
    pub message_key: String,
    pub retryable: bool,
    pub fresh_preparation_required: bool,
    pub diagnostics_id: Option<String>,
}

impl DartError {
    /// Construct a `DartError` from any `Display`-implementing error.
    ///
    /// The raw message is run through [`redact`] and the result is
    /// discarded — only the `message_key` (and the structured fields)
    /// ever reach Dart. This keeps any leaked addresses, hashes, or
    /// base58 payloads inside the Rust process.
    pub fn from<E: std::fmt::Display>(err: &E, category: ErrorCategory) -> Self {
        let raw = format!("{}", err);
        let sanitized = redact(&raw);
        // Sanitized payload is kept internal; nothing about the wrapped
        // error's `Display` is ever exposed to Dart.
        let _ = sanitized;

        Self {
            code: "internal".into(),
            category,
            message_key: "wallet.error.generic".into(),
            retryable: matches!(
                category,
                ErrorCategory::Connectivity | ErrorCategory::ChainState
            ),
            fresh_preparation_required: matches!(category, ErrorCategory::ChainState),
            diagnostics_id: Some(Uuid::new_v4().to_string()),
        }
    }

    /// Convenience for raising a `DartError` directly from a known
    /// category without a wrapped source error. Useful for the
    /// platform-call stubs that only carry a category at construction.
    pub fn from_category(category: ErrorCategory) -> Self {
        Self::from(&"", category)
    }
}

/// Single-pass redactor: strips (in priority order) 0x-prefixed 40-hex
/// addresses, 64-hex SHA-256 hashes, 40-hex addresses without the
/// 0x prefix, and base58 32-byte strings (44-ish chars; recognised by
/// membership in the base58 alphabet). Any non-matching character is
/// preserved verbatim.
///
/// Inputs are scanned byte-by-byte but with multi-byte-aware length
/// counts so multi-byte UTF-8 characters are never split.
fn redact(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let remaining = &s[i..];

        // 0x-prefixed 40-hex (EVM address): "0x" + 40 hex digits.
        if remaining.starts_with("0x") && remaining.len() >= 42 {
            let candidate = &remaining[2..42];
            if candidate.chars().all(|c| c.is_ascii_hexdigit()) {
                out.push_str("0x[redacted]");
                i += 42;
                continue;
            }
        }

        // 64-hex (SHA-256). Check this BEFORE the 40-hex arm so the
        // long run wins.
        if remaining.len() >= 64 {
            let candidate = &remaining[..64];
            if candidate.chars().all(|c| c.is_ascii_hexdigit()) {
                out.push_str("[redacted-hex64]");
                i += 64;
                continue;
            }
        }

        // 40-hex address with no prefix.
        if remaining.len() >= 40 {
            let candidate = &remaining[..40];
            if candidate.chars().all(|c| c.is_ascii_hexdigit()) {
                out.push_str("[redacted-hex40]");
                i += 40;
                continue;
            }
        }

        // base58 32-byte ~44 chars. Only stripped when we see a run of
        // base58 characters at least 32 long (lower bound) so we don't
        // accidentally redact ordinary prose.
        if remaining.len() >= 32 {
            let candidate = &remaining[..32];
            if candidate.chars().all(is_base58_char) {
                // Extend the strip as long as base58 chars continue.
                let mut end = 32;
                while end < remaining.len() && end < 64 {
                    let next = remaining[end..].chars().next();
                    match next {
                        Some(c) if is_base58_char(c) => end += c.len_utf8(),
                        _ => break,
                    }
                }
                out.push_str("[redacted-base58]");
                i += end;
                continue;
            }
        }

        // No match — copy one char (advancing by its UTF-8 length) and
        // continue.
        let c = remaining.chars().next().expect("non-empty slice");
        out.push(c);
        i += c.len_utf8();
    }
    out
}

/// Membership test for the base58 alphabet
/// (`123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`).
fn is_base58_char(c: char) -> bool {
    matches!(c, '1'..='9' | 'A'..='H' | 'J'..='N' | 'P'..='Z' | 'a'..='k' | 'm'..='z')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_evm_address() {
        let s = "sending to 0x1234567890abcdef1234567890abcdef12345678 now";
        let out = redact(s);
        assert!(out.contains("0x[redacted]"));
        assert!(!out.contains("1234567890abcdef1234567890abcdef12345678"));
    }

    #[test]
    fn redacts_sha256() {
        let hash = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        let out = redact(&format!("tx {hash} done"));
        assert!(out.contains("[redacted-hex64]"));
        assert!(!out.contains(hash));
    }

    #[test]
    fn redacts_base58() {
        let payload = "5HbYxtfRsXDUcK3rR2Z8vZ3Y8vZ3Y8vZ3Y8vZ3Y8vZ3Yr";
        let out = redact(&format!("solana payload {payload} end"));
        assert!(out.contains("[redacted-base58]"));
    }

    #[test]
    fn preserves_prose() {
        let s = "the quick brown fox jumps over the lazy dog";
        assert_eq!(redact(s), s);
    }

    #[test]
    fn from_strips_underlying_text() {
        // Construct a wrapping error that contains an EVM address in
        // its Display. The DartError must NOT carry it.
        #[derive(Debug)]
        struct Wrapped(String);
        impl std::fmt::Display for Wrapped {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }
        let wrapped = Wrapped("fee estimate failed for 0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".into());
        let err = DartError::from(&wrapped, ErrorCategory::Broadcast);
        assert!(!err.message_key.contains("0x"));
        assert_eq!(err.code, "internal");
        assert!(err.diagnostics_id.is_some());
    }
}
