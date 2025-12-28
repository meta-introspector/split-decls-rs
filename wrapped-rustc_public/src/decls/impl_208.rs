macro_rules! deps {
    () => {
        ImplPolarity!();
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: ImplPolarity { type T = crate :: ty :: ImplPolarity ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: ty :: ImplPolarity :: * ; match self { Positive => crate :: ty :: ImplPolarity :: Positive , Negative => crate :: ty :: ImplPolarity :: Negative , Reservation => crate :: ty :: ImplPolarity :: Reservation , } } }
    };
}

impl_208!()