// Generated macro for impl_3199 (impl)
macro_rules! Depcrate_sync_mpmcimpl_3199 {
() => {
// Module: crate::sync::mpmc
// Provides: {"impl_3199"}
// Dependencies: {}
# [unstable (feature = "mpmc_channel" , issue = "126840")] impl < T > Clone for Sender < T > { fn clone (& self) -> Self { let flavor = match & self . flavor { SenderFlavor :: Array (chan) => SenderFlavor :: Array (chan . acquire ()) , SenderFlavor :: List (chan) => SenderFlavor :: List (chan . acquire ()) , SenderFlavor :: Zero (chan) => SenderFlavor :: Zero (chan . acquire ()) , } ; Sender { flavor } } }
};
}
