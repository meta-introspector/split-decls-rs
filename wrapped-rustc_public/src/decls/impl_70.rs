macro_rules! deps {
    () => {
        RustcInternal!();
        VariantDef!();
        BridgeTys!();
        InternalCx!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl RustcInternal for VariantDef { type T < 'tcx > = & 'tcx rustc_ty :: VariantDef ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { self . adt_def . internal (tables , tcx) . variant (self . idx . internal (tables , tcx)) } }
    };
}

impl_70!()