// Generated macro for Global (enum)
macro_rules! Depcrate_thread_globalGlobal {
() => {
// Module: crate::thread::global
// Provides: {"Global"}
// Dependencies: {}
# [doc = " Global context."] pub (super) enum Global { # [doc = " [`Window`]."] Window (Window) , # [doc = " [`DedicatedWorkerGlobalScope`]."] Dedicated (DedicatedWorkerGlobalScope) , # [doc = " [`SharedWorkerGlobalScope`]."] Shared (SharedWorkerGlobalScope) , # [doc = " Service worker."] Service (WorkerGlobalScope) , # [doc = " Unknown worker type."] Worker (WorkerGlobalScope) , # [doc = " Worklet."] Worklet , # [doc = " Unknown."] Unknown , }
};
}
