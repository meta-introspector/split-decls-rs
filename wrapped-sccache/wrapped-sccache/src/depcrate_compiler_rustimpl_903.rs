// Generated macro for impl_903 (impl)
macro_rules! Depcrate_compiler_rustimpl_903 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_903"}
// Dependencies: {}
impl IntoArg for ArgTarget { fn into_arg_os_string (self) -> OsString { match self { ArgTarget :: Name (s) => s . into () , ArgTarget :: Path (p) => p . into () , ArgTarget :: Unsure (s) => s , } } fn into_arg_string (self , transformer : PathTransformerFn < '_ >) -> ArgToStringResult { Ok (match self { ArgTarget :: Name (s) => s , ArgTarget :: Path (p) => p . into_arg_string (transformer) ? , ArgTarget :: Unsure (s) => s . into_arg_string (transformer) ? , }) } }
};
}
