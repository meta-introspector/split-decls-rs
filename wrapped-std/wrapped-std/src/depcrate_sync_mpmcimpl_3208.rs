// Generated macro for impl_3208 (impl)
macro_rules! Depcrate_sync_mpmcimpl_3208 {
() => {
// Module: crate::sync::mpmc
// Provides: {"impl_3208"}
// Dependencies: {}
# [unstable (feature = "mpmc_channel" , issue = "126840")] impl < T > Iterator for IntoIter < T > { type Item = T ; fn next (& mut self) -> Option < T > { self . rx . recv () . ok () } }
};
}
