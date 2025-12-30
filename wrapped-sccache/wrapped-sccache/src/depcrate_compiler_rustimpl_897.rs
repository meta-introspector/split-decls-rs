// Generated macro for impl_897 (impl)
macro_rules! Depcrate_compiler_rustimpl_897 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_897"}
// Dependencies: {}
impl IntoArg for ArgUnstable { fn into_arg_os_string (self) -> OsString { let ArgUnstable { opt , value } = self ; if let Some (value) = value { make_os_string ! (opt , "=" , value) } else { make_os_string ! (opt) } } fn into_arg_string (self , transformer : PathTransformerFn < '_ >) -> ArgToStringResult { let ArgUnstable { opt , value } = self ; Ok (if let Some (value) = value { format ! ("{}={}" , opt , value . into_arg_string (transformer) ?) } else { opt }) } }
};
}
