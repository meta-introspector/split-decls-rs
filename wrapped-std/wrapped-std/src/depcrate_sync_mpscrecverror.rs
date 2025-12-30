// Generated macro for RecvError (struct)
macro_rules! Depcrate_sync_mpscRecvError {
() => {
// Module: crate::sync::mpsc
// Provides: {"RecvError"}
// Dependencies: {}
# [doc = " An error returned from the [`recv`] function on a [`Receiver`]."] # [doc = ""] # [doc = " The [`recv`] operation can only fail if the sending half of a"] # [doc = " [`channel`] (or [`sync_channel`]) is disconnected, implying that no further"] # [doc = " messages will ever be received."] # [doc = ""] # [doc = " [`recv`]: Receiver::recv"] # [derive (PartialEq , Eq , Clone , Copy , Debug)] # [stable (feature = "rust1" , since = "1.0.0")] pub struct RecvError ;
};
}
