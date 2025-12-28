macro_rules! deps {
    () => {
        RustcInternal!();
        InternalCx!();
        BridgeTys!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl < T > RustcInternal for & T where T : RustcInternal , { type T < 'tcx > = T :: T < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { (* self) . internal (tables , tcx) } }
    };
}

impl_94!()