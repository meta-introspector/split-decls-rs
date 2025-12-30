// Generated macro for Message (struct)
macro_rules! Depcrate_buffer_messageMessage {
() => {
// Module: crate::buffer::message
// Provides: {"Message"}
// Dependencies: {}
# [doc = " Message sent over buffer"] # [derive (Debug)] pub (crate) struct Message < Request , Fut > { pub (crate) request : Request , pub (crate) tx : Tx < Fut > , pub (crate) span : tracing :: Span , }
};
}
