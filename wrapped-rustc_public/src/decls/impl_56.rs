macro_rules! deps {
    () => {
        GenericArgKind!();
        BridgeTys!();
        InternalCx!();
        RustcInternal!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl RustcInternal for GenericArgKind { type T < 'tcx > = rustc_ty :: GenericArg < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { let arg : rustc_ty :: GenericArg < 'tcx > = match self { GenericArgKind :: Lifetime (reg) => reg . internal (tables , tcx) . into () , GenericArgKind :: Type (ty) => ty . internal (tables , tcx) . into () , GenericArgKind :: Const (cnst) => cnst . internal (tables , tcx) . into () , } ; tcx . lift (arg) . unwrap () } }
    };
}

impl_56!();