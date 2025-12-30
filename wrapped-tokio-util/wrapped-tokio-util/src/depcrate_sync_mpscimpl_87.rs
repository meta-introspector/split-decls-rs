// Generated macro for impl_87 (impl)
macro_rules! Depcrate_sync_mpscimpl_87 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_87"}
// Dependencies: {}
impl < T > PollSendError < T > { # [doc = " Consumes the stored value, if any."] # [doc = ""] # [doc = " If this error was encountered when calling `start_send`/`send_item`, this will be the item"] # [doc = " that the caller attempted to send.  Otherwise, it will be `None`."] pub fn into_inner (self) -> Option < T > { self . 0 } }
};
}
