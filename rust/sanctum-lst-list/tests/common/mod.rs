use lazy_static::lazy_static;
use sanctum_lst_list::{SanctumLst, SanctumLstList};

lazy_static! {
    pub static ref SANCTUM_LST_LIST: SanctumLstList = SanctumLstList::load();
}

pub const SOLANA_RPC_URL: &str = "https://api.mainnet-beta.solana.com";

pub fn find_sanctum_lst_by_symbol_unwrapped(symbol: &str) -> &SanctumLst {
    SANCTUM_LST_LIST
        .sanctum_lst_list
        .iter()
        .find(|lst| lst.symbol == symbol)
        .unwrap()
}
