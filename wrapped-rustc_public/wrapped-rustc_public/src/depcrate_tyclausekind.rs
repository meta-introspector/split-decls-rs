// Generated macro for ClauseKind (enum)
macro_rules! Depcrate_tyClauseKind {
() => {
// Module: crate::ty
// Provides: {"ClauseKind"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum ClauseKind { Trait (TraitPredicate) , RegionOutlives (RegionOutlivesPredicate) , TypeOutlives (TypeOutlivesPredicate) , Projection (ProjectionPredicate) , ConstArgHasType (TyConst , Ty) , WellFormed (TermKind) , ConstEvaluatable (TyConst) , }
};
}
