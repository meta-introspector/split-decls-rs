// Generated macro for impl_247 (impl)
macro_rules! Depcrate_reactor_scopeimpl_247 {
() => {
// Module: crate::reactor::scope
// Provides: {"impl_247"}
// Dependencies: {}
impl < I , O > Stream for ReactorScope < I , O > { type Item = I ; # [inline (always)] fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Pin :: new (& mut self . input_stream) . poll_next (cx) } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { self . input_stream . size_hint () } }
};
}
