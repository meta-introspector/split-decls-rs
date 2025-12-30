// Generated macro for impl_26 (impl)
macro_rules! Depcrate_actor_bridgeimpl_26 {
() => {
// Module: crate::actor::bridge
// Provides: {"impl_26"}
// Dependencies: {}
impl < W > Drop for WorkerBridge < W > where W : Worker , { fn drop (& mut self) { let disconnected = ToWorker :: Disconnected (self . id) ; self . inner . send_message (disconnected) ; } }
};
}
