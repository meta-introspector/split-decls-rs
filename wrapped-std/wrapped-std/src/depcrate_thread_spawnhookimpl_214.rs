// Generated macro for impl_214 (impl)
macro_rules! Depcrate_thread_spawnhookimpl_214 {
() => {
// Module: crate::thread::spawnhook
// Provides: {"impl_214"}
// Dependencies: {}
impl Drop for SpawnHooks { fn drop (& mut self) { let mut next = self . first . take () ; while let Some (SpawnHook { hook , next : n }) = next . and_then (| n | Arc :: into_inner (n)) { drop (hook) ; next = n ; } } }
};
}
