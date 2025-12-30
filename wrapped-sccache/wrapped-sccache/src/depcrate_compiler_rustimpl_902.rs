// Generated macro for impl_902 (impl)
macro_rules! Depcrate_compiler_rustimpl_902 {
() => {
// Module: crate::compiler::rust
// Provides: {"impl_902"}
// Dependencies: {}
impl FromArg for ArgTarget { fn process (arg : OsString) -> ArgParseResult < Self > { if Path :: new (& arg) . extension () . map (| ext | ext == "json") . unwrap_or (false) { return Ok (ArgTarget :: Path (arg . into ())) ; } let mut path = arg . clone () ; path . push (".json") ; if Path :: new (& path) . is_file () { return Ok (ArgTarget :: Unsure (arg)) ; } Ok (ArgTarget :: Name (arg . into_string () . map_err (ArgParseError :: InvalidUnicode) ? ,)) } }
};
}
