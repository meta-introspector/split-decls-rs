// Generated macro for impl_211 (impl)
macro_rules! Depcrate_parser_debugimpl_211 {
() => {
// Module: crate::parser::debug
// Provides: {"impl_211"}
// Dependencies: {}
impl Drop for TraceScope { fn drop (& mut self) { let text = & self . text ; let style = self . style ; drop (self . guard . take ()) ; trace (& format ! ("< {text}") , style) ; } }
};
}
