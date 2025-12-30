// Generated macro for WorkerScope (struct)
macro_rules! Depcrate_actor_scopeWorkerScope {
() => {
// Module: crate::actor::scope
// Provides: {"WorkerScope"}
// Dependencies: {}
# [doc = " This struct holds a reference to a component and to a global scheduler."] pub struct WorkerScope < W : Worker > { state : Shared < WorkerState < W > > , post_msg : Rc < dyn Fn (FromWorker < W >) > , }
};
}
