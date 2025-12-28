macro_rules! deps {
    () => {
        RegionOutlivesPredicate!();
        ProjectionPredicate!();
        TypeOutlivesPredicate!();
        TraitPredicate!();
        TyConst!();
        TermKind!();
        Ty!();
    };
}

macro_rules! ClauseKind {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum ClauseKind { Trait (TraitPredicate) , RegionOutlives (RegionOutlivesPredicate) , TypeOutlives (TypeOutlivesPredicate) , Projection (ProjectionPredicate) , ConstArgHasType (TyConst , Ty) , WellFormed (TermKind) , ConstEvaluatable (TyConst) , }
    };
}

ClauseKind!()