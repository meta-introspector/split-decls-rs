macro_rules! deps {
    () => {
        Unwind!();
    };
}

macro_rules! required_panic_strategy {
    () => {
        deps!();
        fn required_panic_strategy (tcx : TyCtxt < '_ > , _ : LocalCrate) -> Option < PanicStrategy > { if tcx . is_panic_runtime (LOCAL_CRATE) { return Some (tcx . sess . panic_strategy ()) ; } if tcx . sess . panic_strategy () == PanicStrategy :: Abort { return Some (PanicStrategy :: Abort) ; } for def_id in tcx . hir_body_owners () { if tcx . has_ffi_unwind_calls (def_id) { return Some (PanicStrategy :: Unwind) ; } } None }
    };
}

required_panic_strategy!()