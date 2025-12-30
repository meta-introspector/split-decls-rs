// Generated macro for impl_3198 (impl)
macro_rules! Depcrate_sync_mpmcimpl_3198 {
() => {
// Module: crate::sync::mpmc
// Provides: {"impl_3198"}
// Dependencies: {}
# [unstable (feature = "mpmc_channel" , issue = "126840")] impl < T > Drop for Sender < T > { fn drop (& mut self) { unsafe { match & self . flavor { SenderFlavor :: Array (chan) => chan . release (| c | c . disconnect_senders ()) , SenderFlavor :: List (chan) => chan . release (| c | c . disconnect_senders ()) , SenderFlavor :: Zero (chan) => chan . release (| c | c . disconnect ()) , } } } }
};
}
