// Generated macro for is_arg_inside_call (function)
macro_rules! Depcrate_non_fmt_panicis_arg_inside_call {
() => {
// Module: crate::non_fmt_panic
// Provides: {"is_arg_inside_call"}
// Dependencies: {}
fn is_arg_inside_call (arg : Span , call : Span) -> bool { call . contains (arg) && ! call . source_equal (arg) }
};
}
