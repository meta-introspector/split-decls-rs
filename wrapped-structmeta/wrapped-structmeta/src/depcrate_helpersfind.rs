// Generated macro for find (function)
macro_rules! Depcrate_helpersfind {
() => {
// Module: crate::helpers
// Provides: {"find"}
// Dependencies: {}
fn find (names : & [& str] , ident : & Ident) -> Option < usize > { names . iter () . position (| name | ident == name) }
};
}
