// Generated macro for run_spawn_hooks (function)
macro_rules! Depcrate_thread_spawnhookrun_spawn_hooks {
() => {
// Module: crate::thread::spawnhook
// Provides: {"run_spawn_hooks"}
// Dependencies: {}
# [doc = " Runs all the spawn hooks."] # [doc = ""] # [doc = " Called on the parent thread."] # [doc = ""] # [doc = " Returns the functions to be called on the newly spawned thread."] pub (super) fn run_spawn_hooks (thread : & Thread) -> ChildSpawnHooks { if let Ok (hooks) = SPAWN_HOOKS . try_with (| hooks | { let snapshot = hooks . take () ; hooks . set (snapshot . clone ()) ; snapshot }) { let to_run : Vec < _ > = iter :: successors (hooks . first . as_deref () , | hook | hook . next . as_deref ()) . map (| hook | (hook . hook) (thread)) . collect () ; ChildSpawnHooks { hooks , to_run } } else { ChildSpawnHooks :: default () } }
};
}
