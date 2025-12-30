// Generated macro for impl_816 (impl)
macro_rules! Depcrate_pathimpl_816 {
() => {
// Module: crate::path
// Provides: {"impl_816"}
// Dependencies: {}
impl PathArguments { pub fn is_empty (& self) -> bool { match self { PathArguments :: None => true , PathArguments :: AngleBracketed (bracketed) => bracketed . args . is_empty () , PathArguments :: Parenthesized (_) => false , } } pub fn is_none (& self) -> bool { match self { PathArguments :: None => true , PathArguments :: AngleBracketed (_) | PathArguments :: Parenthesized (_) => false , } } }
};
}
