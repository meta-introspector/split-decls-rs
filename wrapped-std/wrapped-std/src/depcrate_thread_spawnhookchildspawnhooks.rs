// Generated macro for ChildSpawnHooks (struct)
macro_rules! Depcrate_thread_spawnhookChildSpawnHooks {
() => {
// Module: crate::thread::spawnhook
// Provides: {"ChildSpawnHooks"}
// Dependencies: {}
# [doc = " The results of running the spawn hooks."] # [doc = ""] # [doc = " This struct is sent to the new thread."] # [doc = " It contains the inherited hooks and the closures to be run."] # [derive (Default)] pub (super) struct ChildSpawnHooks { hooks : SpawnHooks , to_run : Vec < Box < dyn FnOnce () + Send > > , }
};
}
