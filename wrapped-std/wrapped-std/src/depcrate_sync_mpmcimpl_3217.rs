// Generated macro for impl_3217 (impl)
macro_rules! Depcrate_sync_mpmcimpl_3217 {
() => {
// Module: crate::sync::mpmc
// Provides: {"impl_3217"}
// Dependencies: {}
# [unstable (feature = "mpmc_channel" , issue = "126840")] impl < T > Drop for Receiver < T > { fn drop (& mut self) { unsafe { match & self . flavor { ReceiverFlavor :: Array (chan) => chan . release (| c | c . disconnect_receivers ()) , ReceiverFlavor :: List (chan) => chan . release (| c | c . disconnect_receivers ()) , ReceiverFlavor :: Zero (chan) => chan . release (| c | c . disconnect ()) , } } } }
};
}
