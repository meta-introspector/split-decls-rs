macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: RawPtrKind { type T = crate :: mir :: RawPtrKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use mir :: RawPtrKind :: * ; match * self { Const => crate :: mir :: RawPtrKind :: Const , Mut => crate :: mir :: RawPtrKind :: Mut , FakeForPtrMetadata => crate :: mir :: RawPtrKind :: FakeForPtrMetadata , } } }
    };
}

impl_132!()