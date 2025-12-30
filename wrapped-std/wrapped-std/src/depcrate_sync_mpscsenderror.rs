// Generated macro for SendError (struct)
macro_rules! Depcrate_sync_mpscSendError {
() => {
// Module: crate::sync::mpsc
// Provides: {"SendError"}
// Dependencies: {}
# [doc = " An error returned from the [`Sender::send`] or [`SyncSender::send`]"] # [doc = " function on **channel**s."] # [doc = ""] # [doc = " A **send** operation can only fail if the receiving end of a channel is"] # [doc = " disconnected, implying that the data could never be received. The error"] # [doc = " contains the data being sent as a payload so it can be recovered."] # [stable (feature = "rust1" , since = "1.0.0")] # [derive (PartialEq , Eq , Clone , Copy)] pub struct SendError < T > (# [stable (feature = "rust1" , since = "1.0.0")] pub T) ;
};
}
