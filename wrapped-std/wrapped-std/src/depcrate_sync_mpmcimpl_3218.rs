// Generated macro for impl_3218 (impl)
macro_rules! Depcrate_sync_mpmcimpl_3218 {
() => {
// Module: crate::sync::mpmc
// Provides: {"impl_3218"}
// Dependencies: {}
# [unstable (feature = "mpmc_channel" , issue = "126840")] impl < T > Clone for Receiver < T > { fn clone (& self) -> Self { let flavor = match & self . flavor { ReceiverFlavor :: Array (chan) => ReceiverFlavor :: Array (chan . acquire ()) , ReceiverFlavor :: List (chan) => ReceiverFlavor :: List (chan . acquire ()) , ReceiverFlavor :: Zero (chan) => ReceiverFlavor :: Zero (chan . acquire ()) , } ; Receiver { flavor } } }
};
}
