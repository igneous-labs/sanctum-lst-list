use std::{collections::HashSet, ops::Deref, str::FromStr};

use clap::Parser;
use sanctum_lst_list::{SanctumLst, SanctumLstList};
use solana_program::pubkey::Pubkey;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "
Converts sanctum-lst-list.toml into the jup.ag token list csv format:
(Name, Symbol, Mint, Decimals, LogoURI, Validated)
Outputs raw CSV lines to stdout.
See https://github.com/jup-ag/token-list/blob/main/validated-tokens.csv for examples.
"
)]
pub struct Args {
    #[arg(
        long,
        short,
        help = "The `Validated` column value for all the csv entries",
        default_value_t = true
    )]
    pub validated: bool,

    #[arg(
        long,
        short,
        help = "Whether to filter out entries already on `tokens_list`",
        default_value_t = true
    )]
    pub filter: bool,

    #[arg(
        long,
        short,
        help = "URL to fetch jup's validated-tokens.csv file from to filter by",
        default_value_t = String::from("https://raw.githubusercontent.com/jup-ag/token-list/main/validated-tokens.csv")
    )]
    pub tokens_list_url: String,
}

fn main() {
    let Args {
        validated,
        tokens_list_url,
        filter,
    } = Args::parse();
    let SanctumLstList { sanctum_lst_list } = SanctumLstList::load();
    let sanctum_lst_list = if !filter {
        sanctum_lst_list
    } else {
        let fetched_bytes = reqwest::blocking::Client::new()
            .get(tokens_list_url)
            .send()
            .unwrap()
            .bytes()
            .unwrap();
        let mints_on_list: HashSet<Pubkey> = csv::Reader::from_reader(fetched_bytes.deref())
            .records()
            .map(|record_res| {
                let record = record_res.unwrap();
                Pubkey::from_str(record.get(2).unwrap()).unwrap()
            })
            .collect();
        let mut sanctum_lsts_on_list = Vec::new();
        let sanctum_lsts_not_on_list = sanctum_lst_list
            .into_iter()
            .filter(|lst| {
                let is_on_list = mints_on_list.contains(&lst.mint);
                if is_on_list {
                    sanctum_lsts_on_list.push(lst.symbol.clone());
                }
                !is_on_list
            })
            .collect();
        println!("Already on list: ");
        sanctum_lsts_on_list
            .iter()
            .for_each(|symbol| print!("{symbol}, "));
        println!();
        println!();
        sanctum_lsts_not_on_list
    };
    sanctum_lst_list.iter().for_each(
        |SanctumLst {
             mint,
             name,
             symbol,
             logo_uri,
             decimals,
             pool: _,
             ..
         }| println!("{name},{symbol},{mint},{decimals},{logo_uri},{validated}"),
    )
}
