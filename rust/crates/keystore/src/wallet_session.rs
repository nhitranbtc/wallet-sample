use crate::Mnemonic;
use wallet_domain::error::WalletError;
use zeroize::ZeroizeOnDrop;

#[derive(ZeroizeOnDrop)]
pub struct WalletSession {
    pub(crate) seed: [u8; 64],
}

impl WalletSession {
    pub fn from_mnemonic(mnemonic: Mnemonic) -> Result<Self, WalletError> {
        Ok(Self {
            seed: *mnemonic.seed(),
        })
    }
}
