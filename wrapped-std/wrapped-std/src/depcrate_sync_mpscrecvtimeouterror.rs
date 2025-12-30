// Generated macro for RecvTimeoutError (enum)
macro_rules! Depcrate_sync_mpscRecvTimeoutError {
() => {
// Module: crate::sync::mpsc
// Provides: {"RecvTimeoutError"}
// Dependencies: {}
# [doc = " This enumeration is the list of possible errors that made [`recv_timeout`]"] # [doc = " unable to return data when called. This can occur with both a [`channel`] and"] # [doc = " a [`sync_channel`]."] # [doc = ""] # [doc = " [`recv_timeout`]: Receiver::recv_timeout"] # [derive (PartialEq , Eq , Clone , Copy , Debug)] # [stable (feature = "mpsc_recv_timeout" , since = "1.12.0")] pub enum RecvTimeoutError { # [doc = " This **channel** is currently empty, but the **Sender**(s) have not yet"] # [doc = " disconnected, so data may yet become available."] # [stable (feature = "mpsc_recv_timeout" , since = "1.12.0")] Timeout , # [doc = " The **channel**'s sending half has become disconnected, and there will"] # [doc = " never be any more data received on it."] # [stable (feature = "mpsc_recv_timeout" , since = "1.12.0")] Disconnected , }
};
}
