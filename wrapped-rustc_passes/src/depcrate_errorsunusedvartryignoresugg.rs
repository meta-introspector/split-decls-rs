// Generated macro for UnusedVarTryIgnoreSugg (struct)
macro_rules! Depcrate_errorsUnusedVarTryIgnoreSugg {
() => {
// Module: crate::errors
// Provides: {"UnusedVarTryIgnoreSugg"}
// Dependencies: {}
# [derive (Subdiagnostic)] # [multipart_suggestion (passes_suggestion , applicability = "maybe-incorrect")] pub (crate) struct UnusedVarTryIgnoreSugg { # [suggestion_part (code = "{name}: _")] pub shorthands : Vec < Span > , # [suggestion_part (code = "_")] pub non_shorthands : Vec < Span > , pub name : String , }
};
}
