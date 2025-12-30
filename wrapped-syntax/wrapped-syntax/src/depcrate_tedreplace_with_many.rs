// Generated macro for replace_with_many (function)
macro_rules! Depcrate_tedreplace_with_many {
() => {
// Module: crate::ted
// Provides: {"replace_with_many"}
// Dependencies: {}
pub fn replace_with_many (old : impl Element , new : Vec < SyntaxElement >) { let old = old . syntax_element () ; replace_all (old . clone () ..= old , new) ; }
};
}
