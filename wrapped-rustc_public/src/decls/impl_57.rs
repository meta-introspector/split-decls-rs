macro_rules! deps {
    () => {
        RustcInternal!();
        InternalCx!();
        Region!();
        BridgeTys!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl RustcInternal for Region { type T < 'tcx > = rustc_ty :: Region < 'tcx > ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { tcx . lifetimes_re_erased () } }
    };
}

impl_57!()