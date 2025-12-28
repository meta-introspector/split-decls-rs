macro_rules! is_mir_available {
    () => {
        fn is_mir_available (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { tcx . mir_keys (()) . contains (& def_id) }
    };
}

is_mir_available!()