// Generated macro for impl_896 (impl)
macro_rules! Depcrate_compiler_rustimpl_896 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_896"}
// Dependencies: {}
impl FromArg for ArgUnstable { fn process (arg : OsString) -> ArgParseResult < Self > { let (opt , value) = split_os_string_arg (arg , "=") ? ; Ok (ArgUnstable { opt , value }) } }
};
}
