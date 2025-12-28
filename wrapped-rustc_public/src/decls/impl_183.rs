macro_rules! deps {
    () => {
        IntTy!();
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: IntTy { type T = IntTy ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { ty :: IntTy :: Isize => IntTy :: Isize , ty :: IntTy :: I8 => IntTy :: I8 , ty :: IntTy :: I16 => IntTy :: I16 , ty :: IntTy :: I32 => IntTy :: I32 , ty :: IntTy :: I64 => IntTy :: I64 , ty :: IntTy :: I128 => IntTy :: I128 , } } }
    };
}

impl_183!();