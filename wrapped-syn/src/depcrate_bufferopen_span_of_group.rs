// Generated macro for open_span_of_group (function)
macro_rules! Depcrate_bufferopen_span_of_group {
() => {
// Module: crate::buffer
// Provides: {"open_span_of_group"}
// Dependencies: {}
pub (crate) fn open_span_of_group (cursor : Cursor) -> Span { match cursor . entry () { Entry :: Group (group , _) => group . span_open () , _ => cursor . span () , } }
};
}
