// Generated macro for impl_3103 (impl)
macro_rules! Depcrate_sync_mpmc_errorimpl_3103 {
() => {
// Module: crate::sync::mpmc::error
// Provides: {"impl_3103"}
// Dependencies: {}
# [unstable (feature = "mpmc_channel" , issue = "126840")] impl < T > From < SendError < T > > for SendTimeoutError < T > { fn from (err : SendError < T >) -> SendTimeoutError < T > { match err { SendError (e) => SendTimeoutError :: Disconnected (e) , } } }
};
}
