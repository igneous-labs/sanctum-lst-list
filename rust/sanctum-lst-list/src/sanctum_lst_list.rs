use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::SanctumLst;

/// The main struct to deserialize from sanctum-lst-list.toml
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SanctumLstList {
    pub sanctum_lst_list: Vec<SanctumLst>,
}

impl SanctumLstList {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let s = fs::read_to_string(path)?;
        Self::load_from_str(&s)
    }

    pub fn load_from_str(s: &str) -> std::io::Result<Self> {
        toml::from_str(s).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    pub fn load() -> Self {
        // TODO: this include_str might break rust-analyzer or compile-times when sanctum-lst-list.toml gets huge, idk
        let s = include_str!("../sanctum-lst-list.toml");
        Self::load_from_str(s).unwrap()
    }
}
