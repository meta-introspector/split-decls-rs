// Generated macro for TrySendError (enum)
macro_rules! Depcrate_sync_mpscTrySendError {
() => {
// Module: crate::sync::mpsc
// Provides: {"TrySendError"}
// Dependencies: {}
# [doc = " This enumeration is the list of the possible error outcomes for the"] # [doc = " [`try_send`] method."] # [doc = ""] # [doc = " [`try_send`]: SyncSender::try_send"] # [stable (feature = "rust1" , since = "1.0.0")] # [derive (PartialEq , Eq , Clone , Copy)] pub enum TrySendError < T > { # [doc = " The data could not be sent on the [`sync_channel`] because it would require that"] # [doc = " the callee block to send the data."] # [doc = ""] # [doc = " If this is a buffered channel, then the buffer is full at this time. If"] # [doc = " this is not a buffered channel, then there is no [`Receiver`] available to"] # [doc = " acquire the data."] # [stable (feature = "rust1" , since = "1.0.0")] Full (# [stable (feature = "rust1" , since = "1.0.0")] T) , # [doc = " This [`sync_channel`]'s receiving half has disconnected, so the data could not be"] # [doc = " sent. The data is returned back to the callee in this case."] # [stable (feature = "rust1" , since = "1.0.0")] Disconnected (# [stable (feature = "rust1" , since = "1.0.0")] T) , }
};
}
