// Generated macro for CallbackMap (type)
macro_rules! Depcrate_actor_bridgeCallbackMap {
() => {
// Module: crate::actor::bridge
// Provides: {"CallbackMap"}
// Dependencies: {}
pub (crate) type CallbackMap < W > = HashMap < HandlerId , Weak < dyn Fn (< W as Worker > :: Output) > > ;
};
}
