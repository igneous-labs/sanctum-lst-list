use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;

use crate::PoolInfo;

// TODO: add S program
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PoolProgram {
    Lido,
    Marinade,
    ReservePool,
    SanctumSpl,
    Socean,
    Spl,
}

impl Display for PoolProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnsupportedPoolProgramErr;

impl Display for UnsupportedPoolProgramErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for UnsupportedPoolProgramErr {}

impl TryFrom<Pubkey> for PoolProgram {
    type Error = UnsupportedPoolProgramErr;

    fn try_from(value: Pubkey) -> Result<Self, Self::Error> {
        Ok(match value {
            lido_program::ID => Self::Lido,
            marinade_program::ID => Self::Marinade,
            sanctum_reserve_pool_program::ID => Self::ReservePool,
            sanctum_spl_stake_pool_program::ID => Self::SanctumSpl,
            socean_program::ID => Self::Socean,
            spl_stake_pool_program::ID => Self::Spl,
            _ => Err(UnsupportedPoolProgramErr)?,
        })
    }
}

impl From<PoolProgram> for Pubkey {
    fn from(value: PoolProgram) -> Self {
        match value {
            PoolProgram::Lido => lido_program::ID,
            PoolProgram::Marinade => marinade_program::ID,
            PoolProgram::ReservePool => sanctum_reserve_pool_program::ID,
            PoolProgram::SanctumSpl => sanctum_spl_stake_pool_program::ID,
            PoolProgram::Socean => socean_program::ID,
            PoolProgram::Spl => spl_stake_pool_program::ID,
        }
    }
}

impl From<PoolInfo> for PoolProgram {
    fn from(value: PoolInfo) -> Self {
        value.pool_program()
    }
}

pub mod lido_program {
    // .\xc3...xd1 = lido state
    sanctum_macros::declare_program_keys!(
        "CrX7kMhLC3cSsXJdT7JDgqrRVWGnUpX3gfEfxxU2NVLi",
        [
            ("stake_authority", b".\xc3\x8e\xfaG\x07\x0e\x1f\x83\r)\xbc%\xb8\x18\xa5U`rD\x01{\xdf\x9e\"\x9d\xfab\x18\xa2Y\xd1", b"stake_authority")
        ]
    );
}

pub mod marinade_program {
    // u\x11...\xf1 = marinade state addr
    sanctum_macros::declare_program_keys!(
        "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD",
        [
            ("liq_pool_sol_leg", b"u\x11\x9b1u\x80u\x86\xe3\xf4\xa7\xe5\xcd\x0f\x89\x0e\x96\xa7S\xb1\x0f\xcc\xc7h\x1e\x94s\xa0\x082p\xf1", b"liq_sol"),
            ("liq_pool_msol_leg_authority", b"u\x11\x9b1u\x80u\x86\xe3\xf4\xa7\xe5\xcd\x0f\x89\x0e\x96\xa7S\xb1\x0f\xcc\xc7h\x1e\x94s\xa0\x082p\xf1", b"liq_st_sol_authority"),
            ("msol_mint_auth", b"u\x11\x9b1u\x80u\x86\xe3\xf4\xa7\xe5\xcd\x0f\x89\x0e\x96\xa7S\xb1\x0f\xcc\xc7h\x1e\x94s\xa0\x082p\xf1", b"st_mint"),
            ("reserve", b"u\x11\x9b1u\x80u\x86\xe3\xf4\xa7\xe5\xcd\x0f\x89\x0e\x96\xa7S\xb1\x0f\xcc\xc7h\x1e\x94s\xa0\x082p\xf1", b"reserve"),
        ]
    );
}

pub mod sanctum_reserve_pool_program {
    // \xde\...\xc6 = sanctum reserve pool
    sanctum_macros::declare_program_keys!(
        "unpXTU2Ndrc7WWNyEhQWe4udTzSibLPi25SXv2xbCHQ",
        [
            ("sol_reserves", b"\xde\x91\xbbP4tnb;\xfb6\xb7=\xae\"\xa4\x83\xb7\xcf\'\xd2\xad\x83\xfa\x8cx\x05\xa6\xcc`+\xc6"),
            ("fee", b"\xde\x91\xbbP4tnb;\xfb6\xb7=\xae\"\xa4\x83\xb7\xcf\'\xd2\xad\x83\xfa\x8cx\x05\xa6\xcc`+\xc6", b"fee"),
            ("protocol_fee", b"protocol-fee")
        ]
    );
}

pub mod socean_program {
    sanctum_macros::declare_program_keys!("5ocnV1qiCgaQR8Jb8xWnVbApfaygJ8tNoZfgPwsgx9kx", []);
}

pub mod spl_stake_pool_program {
    sanctum_macros::declare_program_keys!("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy", []);
}

pub mod sanctum_spl_stake_pool_program {
    sanctum_macros::declare_program_keys!("SP12tWFxD9oJsVWNavTTBZvMbA6gkAmxtVgxdqvyvhY", []);
}
