// Generated macro for impl_890 (impl)
macro_rules! Depcrate_compiler_rustimpl_890 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_890"}
// Dependencies: {}
impl FromArg for ArgLinkPath { fn process (arg : OsString) -> ArgParseResult < Self > { let (kind , path) = match split_os_string_arg (arg , "=") ? { (kind , Some (path)) => (kind , path) , (path , None) => ("all" . to_owned () , path) , } ; Ok (ArgLinkPath { kind , path : path . into () , }) } }
};
}
