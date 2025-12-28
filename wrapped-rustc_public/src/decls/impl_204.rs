macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
        AliasRelationDirection!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: AliasRelationDirection { type T = crate :: ty :: AliasRelationDirection ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: ty :: AliasRelationDirection :: * ; match self { Equate => crate :: ty :: AliasRelationDirection :: Equate , Subtype => crate :: ty :: AliasRelationDirection :: Subtype , } } }
    };
}

impl_204!();