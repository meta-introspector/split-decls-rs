// Generated macro for insert_raw (function)
macro_rules! Depcrate_tedinsert_raw {
() => {
// Module: crate::ted
// Provides: {"insert_raw"}
// Dependencies: {}
pub fn insert_raw (position : Position , elem : impl Element) { insert_all_raw (position , vec ! [elem . syntax_element ()]) ; }
};
}
