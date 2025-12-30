// Generated macro for impl_422 (impl)
macro_rules! Depcrate_compiler_cimpl_422 {
() => {
// Module: crate::compiler::c
// Provides: {"impl_422"}
// Dependencies: {}
impl ParsedArguments { pub fn output_pretty (& self) -> Cow < '_ , str > { self . outputs . get ("obj") . and_then (| o | o . path . file_name ()) . map (| s | s . to_string_lossy ()) . unwrap_or (Cow :: Borrowed ("Unknown filename")) } }
};
}
