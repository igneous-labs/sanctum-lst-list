use std::error::Error;

use sanctum_lst_list::SanctumLstList;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use spl_token_2022::{extension::StateWithExtensions, state::AccountState};

use crate::common::SOLANA_RPC_URL;

// Copied from
// https://github.com/igneous-labs/stakedex-sdk/blob/master/common/src/pda.rs
// Not worth it pulling in 10 more dependencies just for a PDA fn

mod stakedex_interface {
    solana_program::declare_id!("stkitrT1Uoy18Dk1fTrgPw8W6MVzoCfYoAFT4MLsmhq");
}

const FEE_TOKEN_ACCOUNT_SEED_PREFIX: &[u8; 3] = b"fee";

pub fn fee_token_account_seeds(token_mint: &Pubkey) -> [&[u8]; 2] {
    [FEE_TOKEN_ACCOUNT_SEED_PREFIX, token_mint.as_ref()]
}

fn find_fee_token_acc(mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&fee_token_account_seeds(mint), &stakedex_interface::ID)
}

#[test]
fn verify_all_lsts_have_router_fee_token_acc_created() {
    let rpc = RpcClient::new(SOLANA_RPC_URL);
    let SanctumLstList { sanctum_lst_list } = SanctumLstList::load();
    // just do it sequentially to avoid rpc limits
    for sanctum_lst in sanctum_lst_list {
        if let Err(e) = verify_sanctum_router_fee_token_acc_created(&rpc, &sanctum_lst.mint) {
            panic!("{}: {}", sanctum_lst.symbol, e);
        }
    }
}

fn verify_sanctum_router_fee_token_acc_created(
    rpc: &RpcClient,
    mint: &Pubkey,
) -> Result<(), Box<dyn Error>> {
    let (fee_token_acc, _bump) = find_fee_token_acc(mint);
    let fee_token_acc = rpc.get_account(&fee_token_acc)?;
    let StateWithExtensions { base, .. } =
        StateWithExtensions::<spl_token_2022::state::Account>::unpack(&fee_token_acc.data)?;
    if base.state != AccountState::Initialized {
        return Err(format!(
            "state is {:#?}, expected AccountState::Initialized",
            base.state
        )
        .into());
    }
    Ok(())
}
