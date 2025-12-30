// Generated macro for into_diag_arg_impls (module)
macro_rules! Depcrate_ir_printinto_diag_arg_impls {
() => {
// Module: crate::ir_print
// Provides: {"into_diag_arg_impls"}
// Dependencies: {}
# [cfg (feature = "nightly")] mod into_diag_arg_impls { use rustc_error_messages :: { DiagArgValue , IntoDiagArg } ; use super :: * ; impl < I : Interner > IntoDiagArg for TraitRef < I > { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . to_string () . into_diag_arg (path) } } impl < I : Interner > IntoDiagArg for ExistentialTraitRef < I > { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . to_string () . into_diag_arg (path) } } impl < I : Interner > IntoDiagArg for UnevaluatedConst < I > { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { format ! ("{self:?}") . into_diag_arg (path) } } impl < I : Interner > IntoDiagArg for FnSig < I > { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { format ! ("{self:?}") . into_diag_arg (path) } } impl < I : Interner , T : IntoDiagArg > IntoDiagArg for Binder < I , T > { fn into_diag_arg (self , path : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { self . skip_binder () . into_diag_arg (path) } } impl IntoDiagArg for ClosureKind { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (self . as_str () . into ()) } } }
};
}
