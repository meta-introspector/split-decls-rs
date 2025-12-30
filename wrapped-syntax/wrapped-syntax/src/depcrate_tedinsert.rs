// Generated macro for insert (function)
macro_rules! Depcrate_tedinsert {
() => {
// Module: crate::ted
// Provides: {"insert"}
// Dependencies: {}
pub fn insert (position : Position , elem : impl Element) { insert_all (position , vec ! [elem . syntax_element ()]) ; }
};
}
