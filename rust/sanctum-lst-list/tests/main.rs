use std::collections::HashSet;

use sanctum_lst_list::{SanctumLst, SanctumLstList};

#[test]
fn load() {
    // make sure this doesnt panic
    let SanctumLstList { sanctum_lst_list } = SanctumLstList::load();

    // validate unique mints
    let mut dedup = HashSet::new();
    for SanctumLst { mint, .. } in &sanctum_lst_list {
        if !dedup.insert(mint) {
            panic!("Duplicate mint {mint} found");
        }
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
