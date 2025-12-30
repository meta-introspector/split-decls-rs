// Generated macro for start_of_buffer (function)
macro_rules! Depcrate_bufferstart_of_buffer {
() => {
// Module: crate::buffer
// Provides: {"start_of_buffer"}
// Dependencies: {}
fn start_of_buffer (cursor : Cursor) -> * const Entry { unsafe { match & * cursor . scope { Entry :: End (offset , _) => cursor . scope . offset (* offset) , _ => unreachable ! () , } } }
};
}
