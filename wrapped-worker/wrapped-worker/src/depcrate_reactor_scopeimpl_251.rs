// Generated macro for impl_251 (impl)
macro_rules! Depcrate_reactor_scopeimpl_251 {
() => {
// Module: crate::reactor::scope
// Provides: {"impl_251"}
// Dependencies: {}
impl < I , O > Sink < O > for ReactorScope < I , O > { type Error = Infallible ; fn start_send (mut self : Pin < & mut Self > , item : O) -> Result < () , Self :: Error > { Pin :: new (& mut self . output_sink) . start_send (item) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Pin :: new (& mut self . output_sink) . poll_close (cx) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Pin :: new (& mut self . output_sink) . poll_flush (cx) } fn poll_ready (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Pin :: new (& mut self . output_sink) . poll_flush (cx) } }
};
}
