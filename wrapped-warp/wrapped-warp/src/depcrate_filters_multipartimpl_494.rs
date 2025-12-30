// Generated macro for impl_494 (impl)
macro_rules! Depcrate_filters_multipartimpl_494 {
() => {
// Module: crate::filters::multipart
// Provides: {"impl_494"}
// Dependencies: {}
impl Stream for PartStream { type Item = Result < Bytes , crate :: Error > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . 0 . poll_next (cx) } }
};
}
