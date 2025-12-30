// Generated macro for impl_159 (impl)
macro_rules! Depcrate_layer_testsimpl_159 {
() => {
// Module: crate::layer::tests
// Provides: {"impl_159"}
// Dependencies: {}
impl Subscriber for StringSubscriber { fn register_callsite (& self , _ : & 'static Metadata < 'static >) -> Interest { Interest :: never () } fn enabled (& self , _ : & Metadata < '_ >) -> bool { false } fn new_span (& self , _ : & span :: Attributes < '_ >) -> span :: Id { span :: Id :: from_u64 (1) } fn record (& self , _ : & span :: Id , _ : & span :: Record < '_ >) { } fn record_follows_from (& self , _ : & span :: Id , _ : & span :: Id) { } fn event (& self , _ : & Event < '_ >) { } fn enter (& self , _ : & span :: Id) { } fn exit (& self , _ : & span :: Id) { } }
};
}
