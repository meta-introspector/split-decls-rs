// Generated macro for impl_23 (impl)
macro_rules! Depcrate_actor_bridgeimpl_23 {
() => {
// Module: crate::actor::bridge
// Provides: {"impl_23"}
// Dependencies: {}
impl < W > Drop for WorkerBridgeInner < W > where W : Worker , { fn drop (& mut self) { let destroy = ToWorker :: Destroy ; self . send_message (destroy) ; } }
};
}
