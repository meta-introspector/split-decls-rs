// Generated macro for column_widths (function)
macro_rules! Depcrate_combinator_debug_internalscolumn_widths {
() => {
// Module: crate::combinator::debug::internals
// Provides: {"column_widths"}
// Dependencies: {}
fn column_widths () -> (usize , usize) { let term_width = term_width () ; let min_call_width = 40 ; let min_input_width = 20 ; let decor_width = 3 ; let extra_width = term_width . checked_sub (min_call_width + min_input_width + decor_width) . unwrap_or_default () ; let call_width = min_call_width + 2 * extra_width / 3 ; let input_width = min_input_width + extra_width / 3 ; (call_width , input_width) }
};
}
