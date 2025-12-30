// Generated macro for WorkerLifecycleEvent (enum)
macro_rules! Depcrate_actor_lifecycleWorkerLifecycleEvent {
() => {
// Module: crate::actor::lifecycle
// Provides: {"WorkerLifecycleEvent"}
// Dependencies: {}
# [doc = " Internal Worker lifecycle events"] pub (crate) enum WorkerLifecycleEvent < W : Worker > { # [doc = " Request to create the scope"] Create (WorkerScope < W >) , # [doc = " Internal Worker message"] Message (W :: Message) , # [doc = " External Messages from bridges"] Remote (ToWorker < W >) , # [doc = " Destroy the Worker"] Destroy , }
};
}
