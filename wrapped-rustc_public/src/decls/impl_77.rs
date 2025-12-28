macro_rules! deps {
    () => {
        DynKind!();
        RustcInternal!();
        BridgeTys!();
        InternalCx!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl RustcInternal for DynKind { type T < 'tcx > = rustc_ty :: DynKind ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { DynKind :: Dyn => rustc_ty :: DynKind :: Dyn , } } }
    };
}

impl_77!();