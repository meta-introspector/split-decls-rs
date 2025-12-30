// Generated macro for impl_885 (impl)
macro_rules! Depcrate_compiler_rustimpl_885 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_885"}
// Dependencies: {}
impl IntoArg for ArgCrateTypes { fn into_arg_os_string (self) -> OsString { let ArgCrateTypes { rlib , staticlib , others , } = self ; let mut types : Vec < _ > = others . iter () . map (String :: as_str) . chain (if rlib { Some ("rlib") } else { None }) . chain (if staticlib { Some ("staticlib") } else { None }) . collect () ; types . sort_unstable () ; let types_string = types . join (",") ; types_string . into () } fn into_arg_string (self , _transformer : PathTransformerFn < '_ >) -> ArgToStringResult { let ArgCrateTypes { rlib , staticlib , others , } = self ; let mut types : Vec < _ > = others . iter () . map (String :: as_str) . chain (if rlib { Some ("rlib") } else { None }) . chain (if staticlib { Some ("staticlib") } else { None }) . collect () ; types . sort_unstable () ; let types_string = types . join (",") ; Ok (types_string) } }
};
}
