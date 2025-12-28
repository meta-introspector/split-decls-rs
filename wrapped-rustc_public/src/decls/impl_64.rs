macro_rules! deps {
    () => {
        InternalCx!();
        RustcInternal!();
        BridgeTys!();
        FloatTy!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl RustcInternal for FloatTy { type T < 'tcx > = rustc_ty :: FloatTy ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { FloatTy :: F16 => rustc_ty :: FloatTy :: F16 , FloatTy :: F32 => rustc_ty :: FloatTy :: F32 , FloatTy :: F64 => rustc_ty :: FloatTy :: F64 , FloatTy :: F128 => rustc_ty :: FloatTy :: F128 , } } }
    };
}

impl_64!()