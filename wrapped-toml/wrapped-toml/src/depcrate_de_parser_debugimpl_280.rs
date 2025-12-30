// Generated macro for impl_280 (impl)
macro_rules! Depcrate_de_parser_debugimpl_280 {
() => {
// Module: crate::de::parser::debug
// Provides: {"impl_280"}
// Dependencies: {}
impl Drop for TraceScope { fn drop (& mut self) { let text = & self . text ; let style = self . style ; drop (self . guard . take ()) ; trace (& format ! ("< {text}") , style) ; } }
};
}
