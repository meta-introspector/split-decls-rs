// Generated macro for prepend_child (function)
macro_rules! Depcrate_tedprepend_child {
() => {
// Module: crate::ted
// Provides: {"prepend_child"}
// Dependencies: {}
pub fn prepend_child (node : & (impl Into < SyntaxNode > + Clone) , child : impl Element) { let position = Position :: first_child_of (node) ; insert (position , child) ; }
};
}
