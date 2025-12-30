// Generated macro for impl_210 (impl)
macro_rules! Depcrate_parser_debugimpl_210 {
() => {
// Module: crate::parser::debug
// Provides: {"impl_210"}
// Dependencies: {}
impl TraceScope { pub (crate) fn new (text : impl core :: fmt :: Display) -> Self { let text = text . to_string () ; let style = anstyle :: Style :: new () ; trace (& format ! ("> {text}") , style) ; Self { text , style , guard : DEBUG_DEPTH . scoped () , } } }
};
}
