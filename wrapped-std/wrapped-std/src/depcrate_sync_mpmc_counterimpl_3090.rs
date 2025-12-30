// Generated macro for impl_3090 (impl)
macro_rules! Depcrate_sync_mpmc_counterimpl_3090 {
() => {
// Module: crate::sync::mpmc::counter
// Provides: {"impl_3090"}
// Dependencies: {}
impl < C > ops :: Deref for Sender < C > { type Target = C ; fn deref (& self) -> & C { & self . counter () . chan } }
};
}
