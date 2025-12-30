// Generated macro for impl_893 (impl)
macro_rules! Depcrate_compiler_rustimpl_893 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_893"}
// Dependencies: {}
impl FromArg for ArgCodegen { fn process (arg : OsString) -> ArgParseResult < Self > { let (opt , value) = split_os_string_arg (arg , "=") ? ; Ok (ArgCodegen { opt , value }) } }
};
}
