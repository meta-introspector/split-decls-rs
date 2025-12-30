// Generated macro for impl_228 (impl)
macro_rules! Depcrate_tyimpl_228 {
() => {
// Module: crate::ty
// Provides: {"impl_228"}
// Dependencies: {}
impl PathParameters { pub fn is_empty (& self) -> bool { match * self { PathParameters :: None => true , PathParameters :: AngleBracketed (ref bracketed) => { bracketed . lifetimes . is_empty () && bracketed . types . is_empty () && bracketed . bindings . is_empty () } PathParameters :: Parenthesized (_) => false , } } }
};
}
