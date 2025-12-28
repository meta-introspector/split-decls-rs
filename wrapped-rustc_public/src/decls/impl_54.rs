macro_rules! deps {
    () => {
        DefId!();
        RustcInternal!();
        BridgeTys!();
        InternalCx!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl RustcInternal for DefId { type T < 'tcx > = rustc_span :: def_id :: DefId ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tcx . lift (tables . def_ids [* self]) . unwrap () } }
    };
}

impl_54!()