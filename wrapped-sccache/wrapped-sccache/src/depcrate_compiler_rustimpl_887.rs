// Generated macro for impl_887 (impl)
macro_rules! Depcrate_compiler_rustimpl_887 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_887"}
// Dependencies: {}
impl FromArg for ArgLinkLibrary { fn process (arg : OsString) -> ArgParseResult < Self > { let (kind , name) = match split_os_string_arg (arg , "=") ? { (kind , Some (name)) => (kind , name) , (name , None) => ("dylib" . to_owned () , name) , } ; Ok (ArgLinkLibrary { kind , name }) } }
};
}
