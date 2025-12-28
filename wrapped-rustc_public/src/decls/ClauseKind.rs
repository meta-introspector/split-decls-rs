macro_rules! deps {
    () => {
        RegionOutlivesPredicate!();
        ProjectionPredicate!();
        Ty!();
        TraitPredicate!();
        TermKind!();
        TyConst!();
        TypeOutlivesPredicate!();
    };
}

macro_rules! ClauseKind {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum ClauseKind { Trait (TraitPredicate) , RegionOutlives (RegionOutlivesPredicate) , TypeOutlives (TypeOutlivesPredicate) , Projection (ProjectionPredicate) , ConstArgHasType (TyConst , Ty) , WellFormed (TermKind) , ConstEvaluatable (TyConst) , }
    };
}

ClauseKind!();