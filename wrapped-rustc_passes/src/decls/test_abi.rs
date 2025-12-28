macro_rules! deps {
    () => {
        AbiInvalidAttribute!();
    };
}

macro_rules! test_abi {
    () => {
        deps!();
        pub fn test_abi (tcx : TyCtxt < '_ >) { if ! tcx . features () . rustc_attrs () { return ; } for id in tcx . hir_crate_items (()) . definitions () { for attr in tcx . get_attrs (id , sym :: rustc_abi) { match tcx . def_kind (id) { DefKind :: Fn | DefKind :: AssocFn => { dump_abi_of_fn_item (tcx , id , attr) ; } DefKind :: TyAlias => { dump_abi_of_fn_type (tcx , id , attr) ; } _ => { tcx . dcx () . emit_err (AbiInvalidAttribute { span : tcx . def_span (id) }) ; } } } } }
    };
}

test_abi!();