macro_rules! deps {
    () => {
        FloatTy!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: FloatTy { type T = FloatTy ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { ty :: FloatTy :: F16 => FloatTy :: F16 , ty :: FloatTy :: F32 => FloatTy :: F32 , ty :: FloatTy :: F64 => FloatTy :: F64 , ty :: FloatTy :: F128 => FloatTy :: F128 , } } }
    };
}

impl_185!();