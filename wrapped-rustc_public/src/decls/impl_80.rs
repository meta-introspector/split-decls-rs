macro_rules! deps {
    () => {
        BridgeTys!();
        TermKind!();
        RustcInternal!();
        InternalCx!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl RustcInternal for TermKind { type T < 'tcx > = rustc_ty :: Term < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { TermKind :: Type (ty) => ty . internal (tables , tcx) . into () , TermKind :: Const (cnst) => cnst . internal (tables , tcx) . into () , } } }
    };
}

impl_80!()