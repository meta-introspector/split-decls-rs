macro_rules! deps {
    () => {
        InternalCx!();
        RustcInternal!();
        BridgeTys!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl RustcInternal for AllocId { type T < 'tcx > = rustc_middle :: mir :: interpret :: AllocId ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tcx . lift (tables . alloc_ids [* self]) . unwrap () } }
    };
}

impl_83!()