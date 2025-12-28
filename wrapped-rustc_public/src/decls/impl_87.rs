macro_rules! deps {
    () => {
        RustcInternal!();
        InternalCx!();
        BridgeTys!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl RustcInternal for Safety { type T < 'tcx > = rustc_hir :: Safety ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { Safety :: Unsafe => rustc_hir :: Safety :: Unsafe , Safety :: Safe => rustc_hir :: Safety :: Safe , } } }
    };
}

impl_87!()