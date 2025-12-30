// Generated macro for IntoArg (trait)
macro_rules! Depcrate_compiler_argsIntoArg {
() => {
// Module: crate::compiler::args
// Provides: {"IntoArg"}
// Dependencies: {}
pub trait IntoArg : Sized { fn into_arg_os_string (self) -> OsString ; fn into_arg_string (self , transformer : PathTransformerFn < '_ >) -> ArgToStringResult ; }
};
}
