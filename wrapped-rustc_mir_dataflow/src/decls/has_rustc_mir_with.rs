macro_rules! has_rustc_mir_with {
    () => {
        fn has_rustc_mir_with (tcx : TyCtxt < '_ > , def_id : DefId , name : Symbol) -> Option < MetaItem > { for attr in tcx . get_attrs (def_id , sym :: rustc_mir) { let items = attr . meta_item_list () ; for item in items . iter () . flat_map (| l | l . iter ()) { match item . meta_item () { Some (mi) if mi . has_name (name) => return Some (mi . clone ()) , _ => continue , } } } None }
    };
}

has_rustc_mir_with!()