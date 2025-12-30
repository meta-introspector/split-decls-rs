// Generated macro for impl_3207 (impl)
macro_rules! Depcrate_sync_mpmcimpl_3207 {
() => {
// Module: crate::sync::mpmc
// Provides: {"impl_3207"}
// Dependencies: {}
# [unstable (feature = "mpmc_channel" , issue = "126840")] impl < 'a , T > IntoIterator for & 'a Receiver < T > { type Item = T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
