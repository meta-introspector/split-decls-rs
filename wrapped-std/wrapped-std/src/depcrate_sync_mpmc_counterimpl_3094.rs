// Generated macro for impl_3094 (impl)
macro_rules! Depcrate_sync_mpmc_counterimpl_3094 {
() => {
// Module: crate::sync::mpmc::counter
// Provides: {"impl_3094"}
// Dependencies: {}
impl < C > ops :: Deref for Receiver < C > { type Target = C ; fn deref (& self) -> & C { & self . counter () . chan } }
};
}
