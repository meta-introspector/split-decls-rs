macro_rules! deps {
    () => {
        DuplicateDiagnosticItemInCrate!();
    };
}

macro_rules! report_duplicate_item {
    () => {
        deps!();
        fn report_duplicate_item (tcx : TyCtxt < '_ > , name : Symbol , original_def_id : DefId , item_def_id : DefId ,) { let orig_span = tcx . hir_span_if_local (original_def_id) ; let duplicate_span = tcx . hir_span_if_local (item_def_id) ; tcx . dcx () . emit_err (DuplicateDiagnosticItemInCrate { duplicate_span , orig_span , crate_name : tcx . crate_name (item_def_id . krate) , orig_crate_name : tcx . crate_name (original_def_id . krate) , different_crates : (item_def_id . krate != original_def_id . krate) , name , }) ; }
    };
}

report_duplicate_item!();