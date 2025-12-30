// Generated macro for impl_3254 (impl)
macro_rules! Depcrate_sync_mpscimpl_3254 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_3254"}
// Dependencies: {}
# [stable (feature = "receiver_into_iter" , since = "1.1.0")] impl < T > Iterator for IntoIter < T > { type Item = T ; fn next (& mut self) -> Option < T > { self . rx . recv () . ok () } }
};
}
