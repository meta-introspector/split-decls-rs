macro_rules! deps {
    () => {
        CanonicalQueryInput!();
        QueryInput!();
        Predicate!();
        Interner!();
    };
}

macro_rules! CanonicalInput {
    () => {
        deps!();
        pub type CanonicalInput < I , T = < I as Interner > :: Predicate > = ty :: CanonicalQueryInput < I , QueryInput < I , T > > ;
    };
}

CanonicalInput!()