macro_rules! deps {
    () => {
        SymbolNamesTest!();
    };
}

macro_rules! report_symbol_names {
    () => {
        deps!();
        pub fn report_symbol_names (tcx : TyCtxt < '_ >) { if ! tcx . features () . rustc_attrs () { return ; } tcx . dep_graph . with_ignore (| | { let mut symbol_names = SymbolNamesTest { tcx } ; let crate_items = tcx . hir_crate_items (()) ; for id in crate_items . free_items () { symbol_names . process_attrs (id . owner_id . def_id) ; } for id in crate_items . trait_items () { symbol_names . process_attrs (id . owner_id . def_id) ; } for id in crate_items . impl_items () { symbol_names . process_attrs (id . owner_id . def_id) ; } for id in crate_items . foreign_items () { symbol_names . process_attrs (id . owner_id . def_id) ; } }) }
    };
}

report_symbol_names!()