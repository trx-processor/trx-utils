use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct TxModel {
    pub txid: String,
}
