// Generated macro for PollSender (struct)
macro_rules! Depcrate_sync_mpscPollSender {
() => {
// Module: crate::sync::mpsc
// Provides: {"PollSender"}
// Dependencies: {}
# [doc = " A wrapper around [`mpsc::Sender`] that can be polled."] # [doc = ""] # [doc = " [`mpsc::Sender`]: tokio::sync::mpsc::Sender"] # [derive (Debug)] pub struct PollSender < T > { sender : Option < Sender < T > > , state : State < T > , acquire : PollSenderFuture < T > , }
};
}
