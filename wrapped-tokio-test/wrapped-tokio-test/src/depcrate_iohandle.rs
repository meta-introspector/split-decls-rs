// Generated macro for Handle (struct)
macro_rules! Depcrate_ioHandle {
() => {
// Module: crate::io
// Provides: {"Handle"}
// Dependencies: {}
# [doc = " A handle to send additional actions to the related `Mock`."] # [derive (Debug)] pub struct Handle { tx : mpsc :: UnboundedSender < Action > , }
};
}
