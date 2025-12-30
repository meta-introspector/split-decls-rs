// Generated macro for impl_377 (impl)
macro_rules! Depcrate_compiler_argsimpl_377 {
() => {
// Module: crate::compiler::args
// Provides: {"impl_377"}
// Dependencies: {}
impl IntoArg for OsString { fn into_arg_os_string (self) -> OsString { self } fn into_arg_string (self , _transformer : PathTransformerFn < '_ >) -> ArgToStringResult { self . into_string () . map_err (ArgToStringError :: InvalidUnicode) } }
};
}
