// Generated macro for ImplConstStability (enum)
macro_rules! Depcrate_errorsImplConstStability {
() => {
// Module: crate::errors
// Provides: {"ImplConstStability"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum ImplConstStability { # [note (passes_trait_impl_const_stability_mismatch_impl_stable)] Stable { # [primary_span] span : Span , } , # [note (passes_trait_impl_const_stability_mismatch_impl_unstable)] Unstable { # [primary_span] span : Span , } , }
};
}
