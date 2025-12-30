// Generated macro for impl_279 (impl)
macro_rules! Depcrate_de_parser_debugimpl_279 {
() => {
// Module: crate::de::parser::debug
// Provides: {"impl_279"}
// Dependencies: {}
impl TraceScope { pub (crate) fn new (text : impl core :: fmt :: Display) -> Self { let text = text . to_string () ; let style = anstyle :: Style :: new () ; trace (& format ! ("> {text}") , style) ; Self { text , style , guard : DEBUG_DEPTH . scoped () , } } }
};
}
