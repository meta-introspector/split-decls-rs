// Generated macro for WorkerDestroyHandle (struct)
macro_rules! Depcrate_actor_scopeWorkerDestroyHandle {
() => {
// Module: crate::actor::scope
// Provides: {"WorkerDestroyHandle"}
// Dependencies: {}
# [doc = " A handle that closes the worker when it is dropped."] pub struct WorkerDestroyHandle < W > where W : Worker + 'static , { scope : WorkerScope < W > , }
};
}
