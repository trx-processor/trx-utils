use crate::{Base58Address, TrxWalletContract};

pub struct SrcTrxWallet {
    trx_wallet: TrxWalletContract,
}

impl SrcTrxWallet {
    pub fn get_wallet_address(&self) -> Base58Address {
        self.trx_wallet.address.base58.as_str().into()
    }

    pub fn get_wallet_private_key(&self) -> &str {
        self.trx_wallet.private_key.as_str()
    }
}

impl Into<SrcTrxWallet> for TrxWalletContract {
    fn into(self) -> SrcTrxWallet {
        SrcTrxWallet { trx_wallet: self }
    }
}
