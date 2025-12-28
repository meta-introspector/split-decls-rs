macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_hir :: CoroutineSource { type T = crate :: mir :: CoroutineSource ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_hir :: CoroutineSource ; match self { CoroutineSource :: Block => crate :: mir :: CoroutineSource :: Block , CoroutineSource :: Closure => crate :: mir :: CoroutineSource :: Closure , CoroutineSource :: Fn => crate :: mir :: CoroutineSource :: Fn , } } }
    };
}

impl_225!();