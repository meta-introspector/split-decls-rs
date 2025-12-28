macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
        AliasKind!();
        Opaque!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: AliasTyKind { type T = crate :: ty :: AliasKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { ty :: Projection => crate :: ty :: AliasKind :: Projection , ty :: Inherent => crate :: ty :: AliasKind :: Inherent , ty :: Opaque => crate :: ty :: AliasKind :: Opaque , ty :: Free => crate :: ty :: AliasKind :: Free , } } }
    };
}

impl_163!()