use clap::Parser;
use sanctum_lst_list::{SanctumLst, SanctumLstList};

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
        help = "The `Validated` column value for all the csv entries. Defaults to false if not set.",
        default_value_t = false
    )]
    pub validated: bool,
}

fn main() {
    let Args { validated } = Args::parse();
    let SanctumLstList { sanctum_lst_list } = SanctumLstList::load();
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
