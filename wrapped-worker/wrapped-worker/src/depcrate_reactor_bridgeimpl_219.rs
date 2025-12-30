// Generated macro for impl_219 (impl)
macro_rules! Depcrate_reactor_bridgeimpl_219 {
() => {
// Module: crate::reactor::bridge
// Provides: {"impl_219"}
// Dependencies: {}
impl < R > Sink < < R :: Scope as ReactorScoped > :: Input > for ReactorBridge < R > where R : Reactor + 'static , { type Error = ReactorBridgeSinkError ; fn poll_close (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Err (ReactorBridgeSinkError :: AttemptClosure)) } fn poll_flush (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn poll_ready (self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } fn start_send (self : Pin < & mut Self > , item : < R :: Scope as ReactorScoped > :: Input ,) -> Result < () , Self :: Error > { self . send_input (item) ; Ok (()) } }
};
}
