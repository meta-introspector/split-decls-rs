macro_rules! deps {
    () => {
        TermKind!();
        AliasTerm!();
    };
}

macro_rules! ProjectionPredicate {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct ProjectionPredicate { pub projection_term : AliasTerm , pub term : TermKind , }
    };
}

ProjectionPredicate!();