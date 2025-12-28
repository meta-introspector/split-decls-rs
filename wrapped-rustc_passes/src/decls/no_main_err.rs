macro_rules! deps {
    () => {
        EntryContext!();
        NoMainErr!();
    };
}

macro_rules! no_main_err {
    () => {
        deps!();
        fn no_main_err (tcx : TyCtxt < '_ > , visitor : & EntryContext < '_ >) { let sp = tcx . def_span (CRATE_DEF_ID) ; let mut has_filename = true ; let filename = tcx . sess . local_crate_source_file () . map (| src | src . for_scope (& tcx . sess , RemapPathScopeComponents :: DIAGNOSTICS) . to_path_buf ()) . unwrap_or_else (| | { has_filename = false ; Default :: default () }) ; let main_def_opt = tcx . resolutions (()) . main_def ; let code = E0601 ; let add_teach_note = tcx . sess . teach (code) ; let file_empty = tcx . sess . source_map () . lookup_line (sp . hi ()) . is_err () ; tcx . dcx () . emit_err (NoMainErr { sp , crate_name : tcx . crate_name (LOCAL_CRATE) , has_filename , filename , file_empty , non_main_fns : visitor . non_main_fns . clone () , main_def_opt , add_teach_note , }) ; }
    };
}

no_main_err!()