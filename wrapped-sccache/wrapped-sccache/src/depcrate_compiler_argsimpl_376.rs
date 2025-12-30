// Generated macro for impl_376 (impl)
macro_rules! Depcrate_compiler_argsimpl_376 {
() => {
// Module: crate::compiler::args
// Provides: {"impl_376"}
// Dependencies: {}
impl FromArg for String { fn process (arg : OsString) -> ArgParseResult < Self > { arg . into_string () . map_err (ArgParseError :: InvalidUnicode) } }
};
}
