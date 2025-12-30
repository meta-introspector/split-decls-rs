// Generated macro for impl_3251 (impl)
macro_rules! Depcrate_sync_mpscimpl_3251 {
() => {
// Module: crate::sync::mpsc
// Provides: {"impl_3251"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , T > Iterator for Iter < 'a , T > { type Item = T ; fn next (& mut self) -> Option < T > { self . rx . recv () . ok () } }
};
}
