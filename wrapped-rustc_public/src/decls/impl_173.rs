macro_rules! deps {
    () => {
        AdtKind!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: AdtKind { type T = AdtKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { ty :: AdtKind :: Struct => AdtKind :: Struct , ty :: AdtKind :: Union => AdtKind :: Union , ty :: AdtKind :: Enum => AdtKind :: Enum , } } }
    };
}

impl_173!()