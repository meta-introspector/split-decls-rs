macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_134 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: MutBorrowKind { type T = crate :: mir :: MutBorrowKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: mir :: MutBorrowKind :: * ; match * self { Default => crate :: mir :: MutBorrowKind :: Default , TwoPhaseBorrow => crate :: mir :: MutBorrowKind :: TwoPhaseBorrow , ClosureCapture => crate :: mir :: MutBorrowKind :: ClosureCapture , } } }
    };
}

impl_134!()