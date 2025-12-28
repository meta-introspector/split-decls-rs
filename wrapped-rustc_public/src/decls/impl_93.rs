macro_rules! deps {
    () => {
        RustcInternal!();
        InternalCx!();
        BridgeTys!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl RustcInternal for UnOp { type T < 'tcx > = rustc_middle :: mir :: UnOp ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { UnOp :: Not => rustc_middle :: mir :: UnOp :: Not , UnOp :: Neg => rustc_middle :: mir :: UnOp :: Neg , UnOp :: PtrMetadata => rustc_middle :: mir :: UnOp :: PtrMetadata , } } }
    };
}

impl_93!();