// Generated macro for impl_162 (impl)
macro_rules! Depcrateimpl_162 {
() => {
// Module: crate
// Provides: {"impl_162"}
// Dependencies: {}
impl IntoDiagArg for Level { fn into_diag_arg (self , _ : & mut Option < std :: path :: PathBuf >) -> DiagArgValue { DiagArgValue :: Str (Cow :: Borrowed (self . to_cmd_flag ())) } }
};
}
