// Generated macro for impl_738 (impl)
macro_rules! Depcrate_specimpl_738 {
() => {
// Module: crate::spec
// Provides: {"impl_738"}
// Dependencies: {}
impl IntoDiagArg for PanicStrategy { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Owned (self . desc () . to_string ())) } }
};
}
