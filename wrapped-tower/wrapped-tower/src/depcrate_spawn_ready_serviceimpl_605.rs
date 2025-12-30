// Generated macro for impl_605 (impl)
macro_rules! Depcrate_spawn_ready_serviceimpl_605 {
() => {
// Module: crate::spawn_ready::service
// Provides: {"impl_605"}
// Dependencies: {}
impl < S > Drop for SpawnReady < S > { fn drop (& mut self) { if let Inner :: Future (ref mut task) = self . inner { task . abort () ; } } }
};
}
