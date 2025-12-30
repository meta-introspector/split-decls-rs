// Generated macro for term_width (function)
macro_rules! Depcrate_combinator_debug_internalsterm_width {
() => {
// Module: crate::combinator::debug::internals
// Provides: {"term_width"}
// Dependencies: {}
fn term_width () -> usize { columns_env () . or_else (query_width) . unwrap_or (80) }
};
}
