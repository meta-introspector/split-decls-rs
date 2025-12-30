// Generated macro for columns_env (function)
macro_rules! Depcrate_combinator_debug_internalscolumns_env {
() => {
// Module: crate::combinator::debug::internals
// Provides: {"columns_env"}
// Dependencies: {}
fn columns_env () -> Option < usize > { std :: env :: var ("COLUMNS") . ok () . and_then (| c | c . parse :: < usize > () . ok ()) }
};
}
