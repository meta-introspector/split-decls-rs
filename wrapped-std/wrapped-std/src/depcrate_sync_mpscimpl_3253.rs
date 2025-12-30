// Generated macro for impl_3253 (impl)
macro_rules! Depcrate_sync_mpscimpl_3253 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_3253"}
// Dependencies: {}
# [stable (feature = "receiver_into_iter" , since = "1.1.0")] impl < 'a , T > IntoIterator for & 'a Receiver < T > { type Item = T ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
