// Generated macro for same_scope (function)
macro_rules! Depcrate_buffersame_scope {
() => {
// Module: crate::buffer
// Provides: {"same_scope"}
// Dependencies: {}
pub (crate) fn same_scope (a : Cursor , b : Cursor) -> bool { ptr :: eq (a . scope , b . scope) }
};
}
