// Generated macro for TraitConstStability (enum)
macro_rules! Depcrate_errorsTraitConstStability {
() => {
// Module: crate::errors
// Provides: {"TraitConstStability"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum TraitConstStability { # [note (passes_trait_impl_const_stability_mismatch_trait_stable)] Stable { # [primary_span] span : Span , } , # [note (passes_trait_impl_const_stability_mismatch_trait_unstable)] Unstable { # [primary_span] span : Span , } , }
};
}
