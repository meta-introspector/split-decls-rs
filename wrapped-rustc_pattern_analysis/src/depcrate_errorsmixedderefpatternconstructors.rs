// Generated macro for MixedDerefPatternConstructors (struct)
macro_rules! Depcrate_errorsMixedDerefPatternConstructors {
() => {
// Module: crate::errors
// Provides: {"MixedDerefPatternConstructors"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (pattern_analysis_mixed_deref_pattern_constructors)] pub (crate) struct MixedDerefPatternConstructors < 'tcx > { # [primary_span] pub spans : Vec < Span > , pub smart_pointer_ty : Ty < 'tcx > , # [label (pattern_analysis_deref_pattern_label)] pub deref_pattern_label : Span , # [label (pattern_analysis_normal_constructor_label)] pub normal_constructor_label : Span , }
};
}
