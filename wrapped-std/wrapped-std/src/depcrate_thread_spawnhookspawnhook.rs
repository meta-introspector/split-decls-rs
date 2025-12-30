// Generated macro for SpawnHook (struct)
macro_rules! Depcrate_thread_spawnhookSpawnHook {
() => {
// Module: crate::thread::spawnhook
// Provides: {"SpawnHook"}
// Dependencies: {}
struct SpawnHook { hook : Box < dyn Send + Sync + Fn (& Thread) -> Box < dyn Send + FnOnce () > > , next : Option < Arc < SpawnHook > > , }
};
}
