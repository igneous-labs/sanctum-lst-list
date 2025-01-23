use std::{collections::HashSet, error::Error};

use mpl_token_metadata::accounts::Metadata;
use sanctum_lst_list::SanctumLst;
use serde::Deserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

use crate::common::{find_sanctum_lst_by_symbol_unwrapped, SOLANA_RPC_URL};

// Tests for latest batch

#[tokio::test]
async fn verify_lst_token_metadata_latest_batch() {
    verify_lst_token_metadata_by_symbol("badSOL").await;
}

#[derive(Debug, Deserialize)]
struct OffchainMetadata {
    image: String,
}

async fn verify_lst_token_metadata_by_symbol(symbol: &str) {
    let client = reqwest::Client::new();
    let rpc = RpcClient::new(SOLANA_RPC_URL.to_owned());
    let sanctum_lst = find_sanctum_lst_by_symbol_unwrapped(symbol);
    verify_lst_token_metadata(&client, &rpc, sanctum_lst, &HashSet::new())
        .await
        .unwrap();
}

async fn verify_lst_token_metadata(
    client: &reqwest::Client,
    rpc: &RpcClient,
    SanctumLst {
        mint,
        symbol,
        name,
        logo_uri,
        ..
    }: &SanctumLst,
    ignore_symbols: &HashSet<&str>,
) -> Result<Option<()>, Box<dyn Error>> {
    if ignore_symbols.contains(symbol.as_str()) {
        return Ok(Some(()));
    }
    let (metadata_pk, _bump) = Metadata::find_pda(mint);
    let metadata_acc = match rpc
        .get_account_with_commitment(&metadata_pk, CommitmentConfig::processed())
        .await?
        .value
    {
        Some(a) => a,
        None => return Ok(None),
    };
    let metadata = Metadata::from_bytes(&metadata_acc.data)?;

    for (expected_str, onchain_str, field_name) in [
        (symbol, metadata.symbol, "symbol"),
        (name, metadata.name, "name"),
    ] {
        if *expected_str != onchain_str.replace('\0', "") {
            return Err(format!(
                "Expected {field_name} {expected_str} but onchain is {onchain_str}"
            )
            .into());
        }
    }

    let uri = metadata.uri.replace('\0', "");
    if !uri.is_empty() {
        let OffchainMetadata { image } = client
            .get(uri)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        // use starts_with to account for file extensions
        // e.g. https://arweave.net/ABC vs https://arweave.net/ABC.svg
        if !image.starts_with(logo_uri) && !logo_uri.starts_with(&image) {
            return Err(
                format!("Expected logo URI {logo_uri} but onchain-linked is {image}").into(),
            );
        }
    }

    Ok(Some(()))
}

#[cfg(feature = "test-all")]
#[tokio::test]
async fn verify_all_lsts_token_metadata() {
    let client: &'static reqwest::Client = Box::leak(Box::new(reqwest::Client::new()));
    let rpc = RpcClient::new(SOLANA_RPC_URL.to_owned());
    // just do it sequentially to avoid rpc limits
    let mut no_metadata_lsts = Vec::new();

    let ignore_symbols = [
        "wifSOL",        // name is `dogwifSOL` but onchain is `wifSOL`
        "juicingJupSOL", // symbol onchain is "jjupSOL"
        "fmSOL",         // name is `SolanaFM Staked SOL` but onchain is `fmSOL`
        "dSOL",          // name is `Drift Staked SOL` but onchain is `Drift Staked Sol`
        "stakeSOL",      // name is `Stake City SOL` but onchain is `stakeSOL`
        "pumpkinSOL",    // name is `Pumpkin's Staked SOL` but onchain is `pumpkinSOL`
        "JSOL",          // name is `JPOOL Solana Token` but onchain is `JPool Staked SOL`
        "thugSOL",       // name is `Thugbirdz Staked SOL` but onchain is `ThugBirdz Staked SOL`
        "wenSOL",        // symbol onchain is `WenSOL`
        "dlgtSOL", // name is `Delegate Liquid Staking SOL` but onchain is `Delegate Liquid Staking Token`
        "hausSOL", // name is `StakeHaus Staked SOL` but onchain is `hausSOL`
        "rSOL",    // name is `reflectSOL` but onchain is `Reflect Staked Solana`
        "xSOL",    // name is `ElagabalX Staked SOL` but onchain is `xSOL`
        "stepSOL", // name is `Step Staked SOL` but onchain is `stepSOL`
        "SOL",     // name is `SOL` but onchain is `Wrapped SOL`
        "mSOL",    // name is `Marinade staked SOL` but onchain is `Marinade staked SOL (mSOL)`
        "spikySOL", // malformed linked metadata json
        "pineSOL", // metadata URI links to logo instead of json
        "uwuSOL",  // name is `UwU Staked SOL` but onchain is Uwu Staked SOL`
    ]
    .into_iter()
    .collect();

    for sanctum_lst in crate::common::SANCTUM_LST_LIST.sanctum_lst_list.iter() {
        match verify_lst_token_metadata(client, &rpc, sanctum_lst, &ignore_symbols).await {
            Ok(Some(..)) => (),
            Ok(None) => {
                no_metadata_lsts.push(&sanctum_lst.symbol);
            }
            Err(e) => {
                panic!("{}: {}", sanctum_lst.symbol, e);
            }
        }
    }
    if !no_metadata_lsts.is_empty() {
        eprintln!("LSTs with no onchain token metadata: {no_metadata_lsts:?}");
    }
}
