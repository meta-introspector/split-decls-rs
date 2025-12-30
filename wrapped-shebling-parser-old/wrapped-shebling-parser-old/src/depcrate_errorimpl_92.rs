// Generated macro for impl_92 (impl)
macro_rules! Depcrate_errorimpl_92 {
() => {
// Module: crate::error
// Provides: {"impl_92"}
// Dependencies: {}
impl ParseError { pub (crate) fn line (& self) -> u32 { self . location . line () } pub (crate) fn column (& self) -> usize { self . location . column () } pub (crate) fn notes (& self) -> & [ParseErrorNote] { & self . notes } pub (crate) fn diags (& self) -> & [ParseDiagnostic] { & self . diags } }
};
}
