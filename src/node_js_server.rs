use flurl::IntoFlUrl;
use my_trx_lib::TrxAmount;

use crate::{Base58Address, TrxWallet, TxModel, WalletBalance};

use super::SrcTrxWallet;

#[async_trait::async_trait]
pub trait NodeJsServer {
    async fn get_trx_node_js_url(&self) -> String;

    async fn create_wallet(&self) -> TrxWallet {
        let node_js_url = self.get_trx_node_js_url().await;

        let flurl_response = node_js_url
            .append_path_segment("createWallet")
            .get()
            .await
            .unwrap();

        let response = flurl_response.receive_body().await.unwrap();

        println!("{}", std::str::from_utf8(response.as_slice()).unwrap());

        serde_json::from_slice(response.as_slice()).unwrap()
    }

    async fn transfer_trx(
        &self,
        from_addr: &SrcTrxWallet,
        to_addr: &Base58Address,
        amount: TrxAmount,
    ) -> Result<String, String> {
        let node_js_url = self.get_trx_node_js_url().await;
        let result = node_js_url
            .append_path_segment("sendTrx")
            .append_query_param("fromAddr", from_addr.get_wallet_address().as_str().into()) //Program generated address
            .append_query_param("toAddr", to_addr.as_str().into())
            .append_query_param("amount", amount.as_u64().to_string().into())
            .append_query_param("pk", from_addr.get_wallet_private_key().into())
            .get()
            .await
            .unwrap();

        let result = result.receive_body().await.unwrap();

        let result = String::from_utf8(result).unwrap();

        if result.contains("ContractValidateException") {
            return Err(result);
        }

        let tx: Result<TxModel, _> = serde_json::from_str(result.as_str());

        match tx {
            Ok(result) => Ok(result.txid),
            Err(_) => Err(result),
        }
    }

    async fn get_wallet_balance(&self, address: &Base58Address) -> WalletBalance {
        todo!("Implement me")
    }

    async fn get_wallet_trx_balance(&self, address: &Base58Address) -> i64 {
        todo!("Implement me")
    }
}
