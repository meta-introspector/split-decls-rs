macro_rules! deps {
    () => {
        ClauseKind!();
        TermKind!();
        CoercePredicate!();
        SubtypePredicate!();
        AliasRelationDirection!();
        TyConst!();
    };
}

macro_rules! PredicateKind {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum PredicateKind { Clause (ClauseKind) , DynCompatible (TraitDef) , SubType (SubtypePredicate) , Coerce (CoercePredicate) , ConstEquate (TyConst , TyConst) , Ambiguous , AliasRelate (TermKind , TermKind , AliasRelationDirection) , }
    };
}

PredicateKind!();