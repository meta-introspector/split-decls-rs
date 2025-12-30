// Generated macro for impl_219 (impl)
macro_rules! Depcrate_thread_spawnhookimpl_219 {
() => {
// Module: crate::thread::spawnhook
// Provides: {"impl_219"}
// Dependencies: {}
impl ChildSpawnHooks { pub (super) fn run (self) { SPAWN_HOOKS . set (self . hooks) ; for run in self . to_run { run () ; } } }
};
}
