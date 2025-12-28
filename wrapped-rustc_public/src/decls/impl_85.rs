macro_rules! deps {
    () => {
        BridgeTys!();
        InternalCx!();
        RustcInternal!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl RustcInternal for AdtDef { type T < 'tcx > = rustc_ty :: AdtDef < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { InternalCx :: adt_def (tcx , self . 0 . internal (tables , tcx)) } }
    };
}

impl_85!()