use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Amount(pub u128);

impl Amount {
    pub fn checked_add(self, rhs: Amount) -> Option<Amount> {
        self.0.checked_add(rhs.0).map(Amount)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum AmountError {
    #[error("amount is zero")]
    Zero,
    #[error("precision exceeds chain maximum")]
    PrecisionTooHigh,
    #[error("amount overflows available balance")]
    Overflow,
    #[error("amount cannot be negative")]
    Negative,
}