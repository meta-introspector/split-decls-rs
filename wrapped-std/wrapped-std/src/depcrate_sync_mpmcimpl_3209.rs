// Generated macro for impl_3209 (impl)
macro_rules! Depcrate_sync_mpmcimpl_3209 {
() => {
// Module: crate::sync::mpmc
// Provides: {"impl_3209"}
// Dependencies: {}
# [unstable (feature = "mpmc_channel" , issue = "126840")] impl < T > IntoIterator for Receiver < T > { type Item = T ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> IntoIter < T > { IntoIter { rx : self } } }
};
}
