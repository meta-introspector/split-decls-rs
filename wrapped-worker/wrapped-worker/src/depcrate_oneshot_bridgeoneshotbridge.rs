// Generated macro for OneshotBridge (struct)
macro_rules! Depcrate_oneshot_bridgeOneshotBridge {
() => {
// Module: crate::oneshot::bridge
// Provides: {"OneshotBridge"}
// Dependencies: {}
# [doc = " A connection manager for components interaction with oneshot workers."] # [derive (Debug)] pub struct OneshotBridge < N > where N : Oneshot + 'static , { inner : WorkerBridge < OneshotWorker < N > > , rx : UnboundedReceiver < N :: Output > , }
};
}
