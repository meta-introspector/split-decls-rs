macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: UnOp { type T = crate :: mir :: UnOp ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use rustc_middle :: mir :: UnOp ; match self { UnOp :: Not => crate :: mir :: UnOp :: Not , UnOp :: Neg => crate :: mir :: UnOp :: Neg , UnOp :: PtrMetadata => crate :: mir :: UnOp :: PtrMetadata , } } }
    };
}

impl_150!()