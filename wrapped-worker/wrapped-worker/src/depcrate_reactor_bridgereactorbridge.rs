// Generated macro for ReactorBridge (struct)
macro_rules! Depcrate_reactor_bridgeReactorBridge {
() => {
// Module: crate::reactor::bridge
// Provides: {"ReactorBridge"}
// Dependencies: {}
# [doc = " A connection manager for components interaction with oneshot workers."] # [doc = ""] # [doc = " As this type implements [Stream] + [Sink], it can be splitted with [`StreamExt::split`]."] pub struct ReactorBridge < R > where R : Reactor + 'static , { inner : WorkerBridge < ReactorWorker < R > > , rx : UnboundedReceiver < < R :: Scope as ReactorScoped > :: Output > , }
};
}
