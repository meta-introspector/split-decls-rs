macro_rules! deps {
    () => {
        InternalCx!();
        RustcInternal!();
        BridgeTys!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl RustcInternal for RawPtrKind { type T < 'tcx > = rustc_middle :: mir :: RawPtrKind ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { RawPtrKind :: Mut => rustc_middle :: mir :: RawPtrKind :: Mut , RawPtrKind :: Const => rustc_middle :: mir :: RawPtrKind :: Const , RawPtrKind :: FakeForPtrMetadata => rustc_middle :: mir :: RawPtrKind :: FakeForPtrMetadata , } } }
    };
}

impl_67!();