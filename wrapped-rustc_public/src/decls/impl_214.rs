macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
        Movability!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: Movability { type T = crate :: ty :: Movability ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { ty :: Movability :: Static => crate :: ty :: Movability :: Static , ty :: Movability :: Movable => crate :: ty :: Movability :: Movable , } } }
    };
}

impl_214!();