// Generated macro for div_ceil (function)
macro_rules! Depcrate_event_selectdiv_ceil {
() => {
// Module: crate::event::select
// Provides: {"div_ceil"}
// Dependencies: {}
fn div_ceil (lhs : usize , rhs : usize) -> usize { let d = lhs / rhs ; let r = lhs % rhs ; if r > 0 { d + 1 } else { d } }
};
}
