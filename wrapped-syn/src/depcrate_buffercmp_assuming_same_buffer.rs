// Generated macro for cmp_assuming_same_buffer (function)
macro_rules! Depcrate_buffercmp_assuming_same_buffer {
() => {
// Module: crate::buffer
// Provides: {"cmp_assuming_same_buffer"}
// Dependencies: {}
pub (crate) fn cmp_assuming_same_buffer (a : Cursor , b : Cursor) -> Ordering { a . ptr . cmp (& b . ptr) }
};
}
