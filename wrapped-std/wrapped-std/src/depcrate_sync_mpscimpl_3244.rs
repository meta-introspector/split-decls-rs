// Generated macro for impl_3244 (impl)
macro_rules! Depcrate_sync_mpscimpl_3244 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_3244"}
// Dependencies: {}
impl < T > Sender < T > { # [doc = " Attempts to send a value on this channel, returning it back if it could"] # [doc = " not be sent."] # [doc = ""] # [doc = " A successful send occurs when it is determined that the other end of"] # [doc = " the channel has not hung up already. An unsuccessful send would be one"] # [doc = " where the corresponding receiver has already been deallocated. Note"] # [doc = " that a return value of [`Err`] means that the data will never be"] # [doc = " received, but a return value of [`Ok`] does *not* mean that the data"] # [doc = " will be received. It is possible for the corresponding receiver to"] # [doc = " hang up immediately after this function returns [`Ok`]."] # [doc = ""] # [doc = " This method will never block the current thread."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::mpsc::channel;"] # [doc = ""] # [doc = " let (tx, rx) = channel();"] # [doc = ""] # [doc = " // This send is always successful"] # [doc = " tx.send(1).unwrap();"] # [doc = ""] # [doc = " // This send will fail because the receiver is gone"] # [doc = " drop(rx);"] # [doc = " assert_eq!(tx.send(1).unwrap_err().0, 1);"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] pub fn send (& self , t : T) -> Result < () , SendError < T > > { self . inner . send (t) } }
};
}
