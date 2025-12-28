macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: FakeBorrowKind { type T = crate :: mir :: FakeBorrowKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: mir :: FakeBorrowKind :: * ; match * self { Deep => crate :: mir :: FakeBorrowKind :: Deep , Shallow => crate :: mir :: FakeBorrowKind :: Shallow , } } }
    };
}

impl_135!();