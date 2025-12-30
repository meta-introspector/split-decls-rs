// Generated macro for append_child_raw (function)
macro_rules! Depcrate_tedappend_child_raw {
() => {
// Module: crate::ted
// Provides: {"append_child_raw"}
// Dependencies: {}
pub fn append_child_raw (node : & (impl Into < SyntaxNode > + Clone) , child : impl Element) { let position = Position :: last_child_of (node) ; insert_raw (position , child) ; }
};
}
