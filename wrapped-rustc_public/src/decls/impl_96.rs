macro_rules! deps {
    () => {
        RustcInternal!();
        BridgeTys!();
        InternalCx!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < T > RustcInternal for Vec < T > where T : RustcInternal , { type T < 'tcx > = Vec < T :: T < 'tcx > > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { self . iter () . map (| e | e . internal (tables , tcx)) . collect () } }
    };
}

impl_96!()