use my_trx_lib::TrxAmount;

pub struct WalletBalance {
    pub trx: u64,
    pub usdt: u64,
}

impl WalletBalance {
    pub fn is_empty(&self) -> bool {
        self.trx == 0 && self.usdt == 0
    }

    pub fn get_usdt_balance(&self) -> TrxAmount {
        TrxAmount::from_u64(self.usdt)
    }

    pub fn get_trx_balance(&self) -> TrxAmount {
        TrxAmount::from_u64(self.trx)
    }
}
