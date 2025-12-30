// Generated macro for query_width (function)
macro_rules! Depcrate_combinator_debug_internalsquery_width {
() => {
// Module: crate::combinator::debug::internals
// Provides: {"query_width"}
// Dependencies: {}
fn query_width () -> Option < usize > { use is_terminal_polyfill :: IsTerminal ; if std :: io :: stderr () . is_terminal () { terminal_size :: terminal_size () . map (| (w , _h) | w . 0 . into ()) } else { None } }
};
}
