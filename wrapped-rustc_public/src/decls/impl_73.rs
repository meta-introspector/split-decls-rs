macro_rules! deps {
    () => {
        BridgeTys!();
        RustcInternal!();
        InternalCx!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl RustcInternal for Instance { type T < 'tcx > = rustc_ty :: Instance < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tcx . lift (tables . instances [self . def]) . unwrap () } }
    };
}

impl_73!()