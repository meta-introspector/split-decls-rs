// Generated macro for impl_888 (impl)
macro_rules! Depcrate_compiler_rustimpl_888 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_888"}
// Dependencies: {}
impl IntoArg for ArgLinkLibrary { fn into_arg_os_string (self) -> OsString { let ArgLinkLibrary { kind , name } = self ; make_os_string ! (kind , "=" , name) } fn into_arg_string (self , _transformer : PathTransformerFn < '_ >) -> ArgToStringResult { let ArgLinkLibrary { kind , name } = self ; Ok (format ! ("{}={}" , kind , name)) } }
};
}
