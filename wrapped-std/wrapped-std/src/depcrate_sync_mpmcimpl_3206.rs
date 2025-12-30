// Generated macro for impl_3206 (impl)
macro_rules! Depcrate_sync_mpmcimpl_3206 {
() => {
// Module: crate::sync::mpmc
// Provides: {"impl_3206"}
// Dependencies: {}
# [unstable (feature = "mpmc_channel" , issue = "126840")] impl < 'a , T > Iterator for TryIter < 'a , T > { type Item = T ; fn next (& mut self) -> Option < T > { self . rx . try_recv () . ok () } }
};
}
