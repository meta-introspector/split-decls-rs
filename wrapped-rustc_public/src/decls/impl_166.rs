macro_rules! deps {
    () => {
        Stable!();
        DynKind!();
        BridgeTys!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: DynKind { type T = crate :: ty :: DynKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { ty :: Dyn => crate :: ty :: DynKind :: Dyn , } } }
    };
}

impl_166!();