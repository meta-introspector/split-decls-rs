// Generated macro for WorkerState (struct)
macro_rules! Depcrate_actor_lifecycleWorkerState {
() => {
// Module: crate::actor::lifecycle
// Provides: {"WorkerState"}
// Dependencies: {}
pub (crate) struct WorkerState < W > where W : Worker , { worker : Option < (W , WorkerScope < W >) > , to_destroy : bool , }
};
}
