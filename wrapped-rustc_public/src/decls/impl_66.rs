macro_rules! deps {
    () => {
        BridgeTys!();
        RustcInternal!();
        Movability!();
        InternalCx!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl RustcInternal for Movability { type T < 'tcx > = rustc_ty :: Movability ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { Movability :: Static => rustc_ty :: Movability :: Static , Movability :: Movable => rustc_ty :: Movability :: Movable , } } }
    };
}

impl_66!()