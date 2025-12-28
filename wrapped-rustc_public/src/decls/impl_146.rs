macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: UnwindAction { type T = crate :: mir :: UnwindAction ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: mir :: UnwindAction ; match self { UnwindAction :: Continue => crate :: mir :: UnwindAction :: Continue , UnwindAction :: Unreachable => crate :: mir :: UnwindAction :: Unreachable , UnwindAction :: Terminate (_) => crate :: mir :: UnwindAction :: Terminate , UnwindAction :: Cleanup (bb) => crate :: mir :: UnwindAction :: Cleanup (bb . as_usize ()) , } } }
    };
}

impl_146!();