use serde::*;

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
