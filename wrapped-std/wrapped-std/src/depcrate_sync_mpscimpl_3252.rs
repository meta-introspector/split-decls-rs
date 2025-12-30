// Generated macro for impl_3252 (impl)
macro_rules! Depcrate_sync_mpscimpl_3252 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_3252"}
// Dependencies: {}
# [stable (feature = "receiver_try_iter" , since = "1.15.0")] impl < 'a , T > Iterator for TryIter < 'a , T > { type Item = T ; fn next (& mut self) -> Option < T > { self . rx . try_recv () . ok () } }
};
}
