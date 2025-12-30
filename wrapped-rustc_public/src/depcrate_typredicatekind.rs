// Generated macro for PredicateKind (enum)
macro_rules! Depcrate_tyPredicateKind {
() => {
// Module: crate::ty
// Provides: {"PredicateKind"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum PredicateKind { Clause (ClauseKind) , DynCompatible (TraitDef) , SubType (SubtypePredicate) , Coerce (CoercePredicate) , ConstEquate (TyConst , TyConst) , Ambiguous , AliasRelate (TermKind , TermKind , AliasRelationDirection) , }
};
}
