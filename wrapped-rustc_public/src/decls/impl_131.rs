macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: Mutability { type T = crate :: mir :: Mutability ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_hir :: Mutability :: * ; match * self { Not => crate :: mir :: Mutability :: Not , Mut => crate :: mir :: Mutability :: Mut , } } }
    };
}

impl_131!()