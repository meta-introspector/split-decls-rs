macro_rules! deps {
    () => {
        Layout!();
        RustcInternal!();
        BridgeTys!();
        InternalCx!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl RustcInternal for Layout { type T < 'tcx > = rustc_abi :: Layout < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tcx . lift (tables . layouts [* self]) . unwrap () } }
    };
}

impl_89!();