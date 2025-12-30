// Generated macro for impl_899 (impl)
macro_rules! Depcrate_compiler_rustimpl_899 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_899"}
// Dependencies: {}
impl FromArg for ArgExtern { fn process (arg : OsString) -> ArgParseResult < Self > { if let (name , Some (path)) = split_os_string_arg (arg , "=") ? { Ok (ArgExtern { name , path : path . into () , }) } else { Err (ArgParseError :: Other ("no path for extern")) } } }
};
}
