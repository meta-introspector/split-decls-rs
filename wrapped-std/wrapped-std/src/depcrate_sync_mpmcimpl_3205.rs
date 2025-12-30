// Generated macro for impl_3205 (impl)
macro_rules! Depcrate_sync_mpmcimpl_3205 {
() => {
// Module: crate::sync::mpmc
// Provides: {"impl_3205"}
// Dependencies: {}
# [unstable (feature = "mpmc_channel" , issue = "126840")] impl < 'a , T > Iterator for Iter < 'a , T > { type Item = T ; fn next (& mut self) -> Option < T > { self . rx . recv () . ok () } }
};
}
