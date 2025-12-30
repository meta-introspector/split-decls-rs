// Generated macro for impl_97 (impl)
macro_rules! Depcrate_actor_scopeimpl_97 {
() => {
// Module: crate::actor::scope
// Provides: {"impl_97"}
// Dependencies: {}
impl < W > Drop for WorkerDestroyHandle < W > where W : Worker , { fn drop (& mut self) { self . scope . send (WorkerLifecycleEvent :: Destroy) ; } }
};
}
