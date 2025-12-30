// Generated macro for not_outlives_predicate (function)
macro_rules! Depcrate_normalize_erasing_regionsnot_outlives_predicate {
() => {
// Module: crate::normalize_erasing_regions
// Provides: {"not_outlives_predicate"}
// Dependencies: {}
fn not_outlives_predicate (p : ty :: Predicate < '_ >) -> bool { match p . kind () . skip_binder () { ty :: PredicateKind :: Clause (ty :: ClauseKind :: RegionOutlives (..)) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: TypeOutlives (..)) => false , ty :: PredicateKind :: Clause (ty :: ClauseKind :: Trait (..)) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: Projection (..)) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: HostEffect (..)) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: ConstArgHasType (..)) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: UnstableFeature (_)) | ty :: PredicateKind :: NormalizesTo (..) | ty :: PredicateKind :: AliasRelate (..) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: WellFormed (..)) | ty :: PredicateKind :: DynCompatible (..) | ty :: PredicateKind :: Subtype (..) | ty :: PredicateKind :: Coerce (..) | ty :: PredicateKind :: Clause (ty :: ClauseKind :: ConstEvaluatable (..)) | ty :: PredicateKind :: ConstEquate (..) | ty :: PredicateKind :: Ambiguous => true , } }
};
}
