// Generated macro for impl_378 (impl)
macro_rules! Depcrate_compiler_argsimpl_378 {
() => {
// Module: crate::compiler::args
// Provides: {"impl_378"}
// Dependencies: {}
impl IntoArg for PathBuf { fn into_arg_os_string (self) -> OsString { self . into () } fn into_arg_string (self , transformer : PathTransformerFn < '_ >) -> ArgToStringResult { transformer (& self) . ok_or (ArgToStringError :: FailedPathTransform (self)) } }
};
}
