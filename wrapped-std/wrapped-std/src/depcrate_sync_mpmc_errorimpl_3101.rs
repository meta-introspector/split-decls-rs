// Generated macro for impl_3101 (impl)
macro_rules! Depcrate_sync_mpmc_errorimpl_3101 {
() => {
// Module: crate::sync::mpmc::error
// Provides: {"impl_3101"}
// Dependencies: {}
# [unstable (feature = "mpmc_channel" , issue = "126840")] impl < T > fmt :: Display for SendTimeoutError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { SendTimeoutError :: Timeout (..) => "timed out waiting on send operation" . fmt (f) , SendTimeoutError :: Disconnected (..) => "sending on a disconnected channel" . fmt (f) , } } }
};
}
