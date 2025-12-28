macro_rules! deps {
    () => {
        EntryContext!();
        Node!();
        ExternMain!();
    };
}

macro_rules! configure_main {
    () => {
        deps!();
        fn configure_main (tcx : TyCtxt < '_ > , visitor : & EntryContext < '_ >) -> Option < (DefId , EntryFnType) > { if let Some ((local_def_id , _)) = visitor . rustc_main_fn { let def_id = local_def_id . to_def_id () ; Some ((def_id , EntryFnType :: Main { sigpipe : sigpipe (tcx) })) } else { if let Some (main_def) = tcx . resolutions (()) . main_def && let Some (def_id) = main_def . opt_fn_def_id () { if let Some (def_id) = def_id . as_local () && matches ! (tcx . hir_node_by_def_id (def_id) , Node :: ForeignItem (_)) { tcx . dcx () . emit_err (ExternMain { span : tcx . def_span (def_id) }) ; return None ; } return Some ((def_id , EntryFnType :: Main { sigpipe : sigpipe (tcx) })) ; } no_main_err (tcx , visitor) ; None } }
    };
}

configure_main!();