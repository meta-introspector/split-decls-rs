// Generated macro for impl_891 (impl)
macro_rules! Depcrate_compiler_rustimpl_891 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_891"}
// Dependencies: {}
impl IntoArg for ArgLinkPath { fn into_arg_os_string (self) -> OsString { let ArgLinkPath { kind , path } = self ; make_os_string ! (kind , "=" , path) } fn into_arg_string (self , transformer : PathTransformerFn < '_ >) -> ArgToStringResult { let ArgLinkPath { kind , path } = self ; Ok (format ! ("{}={}" , kind , path . into_arg_string (transformer) ?)) } }
};
}
