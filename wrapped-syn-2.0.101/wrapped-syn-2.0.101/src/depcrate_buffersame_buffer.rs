// Generated macro for same_buffer (function)
macro_rules! Depcrate_buffersame_buffer {
() => {
// Module: crate::buffer
// Provides: {"same_buffer"}
// Dependencies: {}
pub (crate) fn same_buffer (a : Cursor , b : Cursor) -> bool { ptr :: eq (start_of_buffer (a) , start_of_buffer (b)) }
};
}
