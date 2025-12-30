// Generated macro for WorkerBridge (struct)
macro_rules! Depcrate_actor_bridgeWorkerBridge {
() => {
// Module: crate::actor::bridge
// Provides: {"WorkerBridge"}
// Dependencies: {}
# [doc = " A connection manager for components interaction with workers."] pub struct WorkerBridge < W > where W : Worker , { inner : Rc < WorkerBridgeInner < W > > , id : HandlerId , _worker : PhantomData < W > , _cb : Option < Rc < dyn Fn (W :: Output) > > , }
};
}
