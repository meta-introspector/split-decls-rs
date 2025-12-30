// Generated macro for impl_22 (impl)
macro_rules! Depcrate_actor_bridgeimpl_22 {
() => {
// Module: crate::actor::bridge
// Provides: {"impl_22"}
// Dependencies: {}
impl < W > WorkerBridgeInner < W > where W : Worker , { # [doc = " Send a message to the worker, queuing the message if necessary"] fn send_message (& self , msg : ToWorker < W >) { let mut pending_queue = self . pending_queue . borrow_mut () ; match pending_queue . as_mut () { Some (m) => { m . push (msg) ; } None => { (self . post_msg) (msg) ; } } } }
};
}
