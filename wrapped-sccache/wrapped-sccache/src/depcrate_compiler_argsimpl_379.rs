// Generated macro for impl_379 (impl)
macro_rules! Depcrate_compiler_argsimpl_379 {
() => {
// Module: crate::compiler::args
// Provides: {"impl_379"}
// Dependencies: {}
impl IntoArg for String { fn into_arg_os_string (self) -> OsString { self . into () } fn into_arg_string (self , _transformer : PathTransformerFn < '_ >) -> ArgToStringResult { Ok (self) } }
};
}
