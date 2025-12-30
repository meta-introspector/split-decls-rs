// Generated macro for WorkerRunnable (struct)
macro_rules! Depcrate_actor_lifecycleWorkerRunnable {
() => {
// Module: crate::actor::lifecycle
// Provides: {"WorkerRunnable"}
// Dependencies: {}
pub (crate) struct WorkerRunnable < W : Worker > { pub state : Shared < WorkerState < W > > , pub event : WorkerLifecycleEvent < W > , }
};
}
