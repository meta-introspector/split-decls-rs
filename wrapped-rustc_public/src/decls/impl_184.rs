macro_rules! deps {
    () => {
        Stable!();
        UintTy!();
        BridgeTys!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: UintTy { type T = UintTy ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { ty :: UintTy :: Usize => UintTy :: Usize , ty :: UintTy :: U8 => UintTy :: U8 , ty :: UintTy :: U16 => UintTy :: U16 , ty :: UintTy :: U32 => UintTy :: U32 , ty :: UintTy :: U64 => UintTy :: U64 , ty :: UintTy :: U128 => UintTy :: U128 , } } }
    };
}

impl_184!()