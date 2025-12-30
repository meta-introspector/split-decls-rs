// Generated macro for TraitImplConstStabilityMismatch (struct)
macro_rules! Depcrate_errorsTraitImplConstStabilityMismatch {
() => {
// Module: crate::errors
// Provides: {"TraitImplConstStabilityMismatch"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_trait_impl_const_stability_mismatch)] pub (crate) struct TraitImplConstStabilityMismatch { # [primary_span] pub span : Span , # [subdiagnostic] pub impl_stability : ImplConstStability , # [subdiagnostic] pub trait_stability : TraitConstStability , }
};
}
