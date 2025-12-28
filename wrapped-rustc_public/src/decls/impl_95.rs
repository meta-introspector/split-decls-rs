macro_rules! deps {
    () => {
        RustcInternal!();
        BridgeTys!();
        InternalCx!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < T > RustcInternal for Option < T > where T : RustcInternal , { type T < 'tcx > = Option < T :: T < 'tcx > > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { self . as_ref () . map (| inner | inner . internal (tables , tcx)) } }
    };
}

impl_95!();