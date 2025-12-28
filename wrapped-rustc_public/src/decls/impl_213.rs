macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: Variance { type T = crate :: mir :: Variance ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { ty :: Bivariant => crate :: mir :: Variance :: Bivariant , ty :: Contravariant => crate :: mir :: Variance :: Contravariant , ty :: Covariant => crate :: mir :: Variance :: Covariant , ty :: Invariant => crate :: mir :: Variance :: Invariant , } } }
    };
}

impl_213!()