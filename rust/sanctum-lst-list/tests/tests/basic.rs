use std::collections::HashSet;

use sanctum_lst_list::{SanctumLst, SanctumLstList};

#[test]
fn load_and_dedup() {
    // make sure this doesnt panic
    let SanctumLstList { sanctum_lst_list } = SanctumLstList::load();

    // validate unique:
    // - mint
    // - name
    // - symbol
    let mut dedup_mint = HashSet::new();
    /*
    let mut dedup_name = HashSet::new();
    let mut dedup_symbol = HashSet::new();
     */
    for SanctumLst { mint, .. } in &sanctum_lst_list {
        if !dedup_mint.insert(mint) {
            panic!("Duplicate mint {mint} found");
        }
        // names and symbols are now no longer unique
        // due to how many LSTs there are
        /*
        if !dedup_name.insert(name) {
            panic!("Duplicate name {name} found");
        }
        if !dedup_symbol.insert(symbol) {
            panic!("Duplicate symbol {symbol} found");
        }
         */
    }
    println!("{sanctum_lst_list:#?}");
    // TODO: sample 1 from each enum variant to check the fields values
    // to make sure everything is deserialized as expected
}

#[test]
fn it_serializes() {
    let loaded = SanctumLstList::load();
    let serialized = toml::to_string(&loaded).unwrap();
    println!("{}", serialized);
}
