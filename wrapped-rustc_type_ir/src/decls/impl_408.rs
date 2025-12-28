macro_rules! deps {
    () => {
        ClauseKind!();
        Interner!();
    };
}

macro_rules! impl_408 {
    () => {
        deps!();
        impl < I : Interner > fmt :: Debug for ClauseKind < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { ClauseKind :: ConstArgHasType (ct , ty) => write ! (f , "ConstArgHasType({ct:?}, {ty:?})") , ClauseKind :: HostEffect (data) => data . fmt (f) , ClauseKind :: Trait (a) => a . fmt (f) , ClauseKind :: RegionOutlives (pair) => pair . fmt (f) , ClauseKind :: TypeOutlives (pair) => pair . fmt (f) , ClauseKind :: Projection (pair) => pair . fmt (f) , ClauseKind :: WellFormed (data) => write ! (f , "WellFormed({data:?})") , ClauseKind :: ConstEvaluatable (ct) => { write ! (f , "ConstEvaluatable({ct:?})") } ClauseKind :: UnstableFeature (feature_name) => { write ! (f , "UnstableFeature({feature_name:?})") } } } }
    };
}

impl_408!()