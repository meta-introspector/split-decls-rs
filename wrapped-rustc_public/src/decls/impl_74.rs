macro_rules! deps {
    () => {
        InternalCx!();
        BridgeTys!();
        RustcInternal!();
        DefId!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl RustcInternal for StaticDef { type T < 'tcx > = rustc_span :: def_id :: DefId ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { self . 0 . internal (tables , tcx) } }
    };
}

impl_74!();