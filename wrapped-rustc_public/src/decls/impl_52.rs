macro_rules! deps {
    () => {
        RustcInternal!();
        BridgeTys!();
        DefId!();
        InternalCx!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl RustcInternal for CrateItem { type T < 'tcx > = rustc_span :: def_id :: DefId ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { self . 0 . internal (tables , tcx) } }
    };
}

impl_52!()