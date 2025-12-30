// Generated macro for impl_57 (impl)
macro_rules! Depcrate_check_attrimpl_57 {
() => {
// Module: crate::check_attr
// Provides: {"impl_57"}
// Dependencies: {}
impl IntoDiagArg for ProcMacroKind { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> rustc_errors :: DiagArgValue { match self { ProcMacroKind :: Attribute => "attribute proc macro" , ProcMacroKind :: Derive => "derive proc macro" , ProcMacroKind :: FunctionLike => "function-like proc macro" , } . into_diag_arg (& mut None) } }
};
}
