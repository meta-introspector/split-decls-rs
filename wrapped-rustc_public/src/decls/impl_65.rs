macro_rules! deps {
    () => {
        BridgeTys!();
        RustcInternal!();
        InternalCx!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl RustcInternal for Mutability { type T < 'tcx > = rustc_ty :: Mutability ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { Mutability :: Not => rustc_ty :: Mutability :: Not , Mutability :: Mut => rustc_ty :: Mutability :: Mut , } } }
    };
}

impl_65!()