macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: BorrowKind { type T = crate :: mir :: BorrowKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_middle :: mir :: BorrowKind :: * ; match * self { Shared => crate :: mir :: BorrowKind :: Shared , Fake (kind) => crate :: mir :: BorrowKind :: Fake (kind . stable (tables , cx)) , Mut { kind } => crate :: mir :: BorrowKind :: Mut { kind : kind . stable (tables , cx) } , } } }
    };
}

impl_133!()