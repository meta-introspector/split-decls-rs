macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_hir :: Safety { type T = crate :: mir :: Safety ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { rustc_hir :: Safety :: Unsafe => crate :: mir :: Safety :: Unsafe , rustc_hir :: Safety :: Safe => crate :: mir :: Safety :: Safe , } } }
    };
}

impl_223!()