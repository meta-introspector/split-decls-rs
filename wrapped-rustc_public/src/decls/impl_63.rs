macro_rules! deps {
    () => {
        BridgeTys!();
        RustcInternal!();
        InternalCx!();
        UintTy!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl RustcInternal for UintTy { type T < 'tcx > = rustc_ty :: UintTy ; fn internal < 'tcx > (& self , _tables : & mut Tables < '_ , BridgeTys > , _tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { UintTy :: Usize => rustc_ty :: UintTy :: Usize , UintTy :: U8 => rustc_ty :: UintTy :: U8 , UintTy :: U16 => rustc_ty :: UintTy :: U16 , UintTy :: U32 => rustc_ty :: UintTy :: U32 , UintTy :: U64 => rustc_ty :: UintTy :: U64 , UintTy :: U128 => rustc_ty :: UintTy :: U128 , } } }
    };
}

impl_63!();