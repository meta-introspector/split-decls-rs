// Generated macro for impl_28 (impl)
macro_rules! Depcrate_futureimpl_28 {
() => {
// Module: crate::future
// Provides: {"impl_28"}
// Dependencies: {}
impl Stream for IntervalStream { type Item = () ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context) -> Poll < Option < Self :: Item > > { Stream :: poll_next (Pin :: new (& mut self . receiver) , cx) } }
};
}
