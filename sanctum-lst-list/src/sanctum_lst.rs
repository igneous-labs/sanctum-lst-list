use serde::{Deserialize, Serialize};
use serde_with::{As, DisplayFromStr};
use solana_program::pubkey::Pubkey;

use crate::PoolProgram;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(tag = "program")]
pub enum PoolInfo {
    Lido,
    Marinade,
    ReservePool,
    Socean {
        #[serde(with = "As::<DisplayFromStr>")]
        pool: Pubkey,

        #[serde(with = "As::<DisplayFromStr>")]
        validator_list: Pubkey,
    },
    Spl {
        #[serde(with = "As::<DisplayFromStr>")]
        pool: Pubkey,

        #[serde(with = "As::<DisplayFromStr>")]
        validator_list: Pubkey,
    },
    SanctumSpl {
        #[serde(with = "As::<DisplayFromStr>")]
        pool: Pubkey,

        #[serde(with = "As::<DisplayFromStr>")]
        validator_list: Pubkey,
    },
}

impl PoolInfo {
    pub fn pool_program(&self) -> PoolProgram {
        match self {
            PoolInfo::Lido => PoolProgram::Lido,
            PoolInfo::Marinade => PoolProgram::Marinade,
            PoolInfo::ReservePool => PoolProgram::ReservePool,
            PoolInfo::SanctumSpl { .. } => PoolProgram::SanctumSpl,
            PoolInfo::Socean { .. } => PoolProgram::Socean,
            PoolInfo::Spl { .. } => PoolProgram::Spl,
        }
    }
}

/// The entry for a single supported LST
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SanctumLst {
    pub name: String,
    pub symbol: String,
    pub logo_uri: String,

    #[serde(with = "As::<DisplayFromStr>")]
    pub mint: Pubkey,
    pub decimals: u8,
    pub pool: PoolInfo,
}
