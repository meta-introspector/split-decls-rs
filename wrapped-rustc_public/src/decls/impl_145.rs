macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: RetagKind { type T = crate :: mir :: RetagKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: mir :: RetagKind ; match self { RetagKind :: FnEntry => crate :: mir :: RetagKind :: FnEntry , RetagKind :: TwoPhase => crate :: mir :: RetagKind :: TwoPhase , RetagKind :: Raw => crate :: mir :: RetagKind :: Raw , RetagKind :: Default => crate :: mir :: RetagKind :: Default , } } }
    };
}

impl_145!()