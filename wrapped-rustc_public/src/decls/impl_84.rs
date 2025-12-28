macro_rules! deps {
    () => {
        BridgeTys!();
        ClosureKind!();
        InternalCx!();
        RustcInternal!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl RustcInternal for ClosureKind { type T < 'tcx > = rustc_ty :: ClosureKind ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { ClosureKind :: Fn => rustc_ty :: ClosureKind :: Fn , ClosureKind :: FnMut => rustc_ty :: ClosureKind :: FnMut , ClosureKind :: FnOnce => rustc_ty :: ClosureKind :: FnOnce , } } }
    };
}

impl_84!()