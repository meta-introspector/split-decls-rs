// Generated macro for append_child (function)
macro_rules! Depcrate_tedappend_child {
() => {
// Module: crate::ted
// Provides: {"append_child"}
// Dependencies: {}
pub fn append_child (node : & (impl Into < SyntaxNode > + Clone) , child : impl Element) { let position = Position :: last_child_of (node) ; insert (position , child) ; }
};
}
