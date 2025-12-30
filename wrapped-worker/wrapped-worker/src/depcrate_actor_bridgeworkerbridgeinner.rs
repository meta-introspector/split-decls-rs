// Generated macro for WorkerBridgeInner (struct)
macro_rules! Depcrate_actor_bridgeWorkerBridgeInner {
() => {
// Module: crate::actor::bridge
// Provides: {"WorkerBridgeInner"}
// Dependencies: {}
struct WorkerBridgeInner < W > where W : Worker , { pending_queue : Shared < Option < ToWorkerQueue < W > > > , callbacks : Shared < CallbackMap < W > > , post_msg : Rc < dyn Fn (ToWorker < W >) > , }
};
}
