// Generated macro for replace (function)
macro_rules! Depcrate_tedreplace {
() => {
// Module: crate::ted
// Provides: {"replace"}
// Dependencies: {}
pub fn replace (old : impl Element , new : impl Element) { replace_with_many (old , vec ! [new . syntax_element ()]) ; }
};
}
