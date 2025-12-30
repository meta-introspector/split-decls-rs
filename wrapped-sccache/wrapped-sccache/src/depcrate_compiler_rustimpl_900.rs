// Generated macro for impl_900 (impl)
macro_rules! Depcrate_compiler_rustimpl_900 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_900"}
// Dependencies: {}
impl IntoArg for ArgExtern { fn into_arg_os_string (self) -> OsString { let ArgExtern { name , path } = self ; make_os_string ! (name , "=" , path) } fn into_arg_string (self , transformer : PathTransformerFn < '_ >) -> ArgToStringResult { let ArgExtern { name , path } = self ; Ok (format ! ("{}={}" , name , path . into_arg_string (transformer) ?)) } }
};
}
