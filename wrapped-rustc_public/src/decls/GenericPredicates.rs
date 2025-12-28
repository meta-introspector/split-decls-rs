macro_rules! deps {
    () => {
        Span!();
        PredicateKind!();
    };
}

macro_rules! GenericPredicates {
    () => {
        deps!();
        pub struct GenericPredicates { pub parent : Option < TraitDef > , pub predicates : Vec < (PredicateKind , Span) > , }
    };
}

GenericPredicates!();