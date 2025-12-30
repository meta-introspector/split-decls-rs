// Generated macro for impl_3255 (impl)
macro_rules! Depcrate_sync_mpscimpl_3255 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_3255"}
// Dependencies: {}
# [stable (feature = "receiver_into_iter" , since = "1.1.0")] impl < T > IntoIterator for Receiver < T > { type Item = T ; type IntoIter = IntoIter < T > ; fn into_iter (self) -> IntoIter < T > { IntoIter { rx : self } } }
};
}
