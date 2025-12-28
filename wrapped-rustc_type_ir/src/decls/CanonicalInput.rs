macro_rules! deps {
    () => {
        Predicate!();
        QueryInput!();
        Interner!();
        CanonicalQueryInput!();
    };
}

macro_rules! CanonicalInput {
    () => {
        deps!();
        pub type CanonicalInput < I , T = < I as Interner > :: Predicate > = ty :: CanonicalQueryInput < I , QueryInput < I , T > > ;
    };
}

CanonicalInput!();