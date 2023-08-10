use serde::*;

use crate::Base58Address;

#[derive(Serialize, Deserialize)]
pub struct TrxWalletContract {
    #[serde(rename = "privateKey")]
    pub private_key: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,

    pub address: TrxAddressContract,
}

#[derive(Serialize, Deserialize)]
pub struct TrxAddressContract {
    pub base58: String,
    pub hex: String,
}

impl TrxAddressContract {
    pub fn get_base_58_address(&self) -> Base58Address {
        Base58Address::new(self.base58.clone())
    }
}
