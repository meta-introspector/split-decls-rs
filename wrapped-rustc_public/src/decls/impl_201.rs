macro_rules! deps {
    () => {
        ClosureKind!();
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_201 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: ClosureKind { type T = crate :: ty :: ClosureKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: ty :: ClosureKind :: * ; match self { Fn => crate :: ty :: ClosureKind :: Fn , FnMut => crate :: ty :: ClosureKind :: FnMut , FnOnce => crate :: ty :: ClosureKind :: FnOnce , } } }
    };
}

impl_201!()