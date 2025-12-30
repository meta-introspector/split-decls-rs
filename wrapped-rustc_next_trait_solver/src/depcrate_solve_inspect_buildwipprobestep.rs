// Generated macro for WipProbeStep (enum)
macro_rules! Depcrate_solve_inspect_buildWipProbeStep {
() => {
// Module: crate::solve::inspect::build
// Provides: {"WipProbeStep"}
// Dependencies: {}
# [derive_where (PartialEq , Debug ; I : Interner)] enum WipProbeStep < I : Interner > { AddGoal (GoalSource , inspect :: CanonicalState < I , Goal < I , I :: Predicate > >) , NestedProbe (WipProbe < I >) , MakeCanonicalResponse { shallow_certainty : Certainty } , RecordImplArgs { impl_args : inspect :: CanonicalState < I , I :: GenericArgs > } , }
};
}
