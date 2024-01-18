use sanctum_lst_list::SanctumLstList;

#[test]
fn load() {
    // make sure this doesnt panic
    let _list = SanctumLstList::load();
    // TODO: sample 1 from each enum variant to make sure everything works
    // as expected
    //println!("{_list:?}");
}
