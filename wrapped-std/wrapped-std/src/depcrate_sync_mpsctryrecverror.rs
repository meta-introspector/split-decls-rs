// Generated macro for TryRecvError (enum)
macro_rules! Depcrate_sync_mpscTryRecvError {
() => {
// Module: crate::sync::mpsc
// Provides: {"TryRecvError"}
// Dependencies: {}
# [doc = " This enumeration is the list of the possible reasons that [`try_recv`] could"] # [doc = " not return data when called. This can occur with both a [`channel`] and"] # [doc = " a [`sync_channel`]."] # [doc = ""] # [doc = " [`try_recv`]: Receiver::try_recv"] # [derive (PartialEq , Eq , Clone , Copy , Debug)] # [stable (feature = "rust1" , since = "1.0.0")] pub enum TryRecvError { # [doc = " This **channel** is currently empty, but the **Sender**(s) have not yet"] # [doc = " disconnected, so data may yet become available."] # [stable (feature = "rust1" , since = "1.0.0")] Empty , # [doc = " The **channel**'s sending half has become disconnected, and there will"] # [doc = " never be any more data received on it."] # [stable (feature = "rust1" , since = "1.0.0")] Disconnected , }
};
}
