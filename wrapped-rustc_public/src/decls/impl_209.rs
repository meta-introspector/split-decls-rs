macro_rules! deps {
    () => {
        PredicatePolarity!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: PredicatePolarity { type T = crate :: ty :: PredicatePolarity ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: ty :: PredicatePolarity :: * ; match self { Positive => crate :: ty :: PredicatePolarity :: Positive , Negative => crate :: ty :: PredicatePolarity :: Negative , } } }
    };
}

impl_209!();