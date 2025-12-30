// Generated macro for impl_27 (impl)
macro_rules! Depcrate_test_helpersimpl_27 {
() => {
// Module: crate::test_helpers
// Provides: {"impl_27"}
// Dependencies: {}
impl http_body :: Body for Body { type Data = Bytes ; type Error = BoxError ; fn poll_frame (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { Pin :: new (& mut self . 0) . poll_frame (cx) } fn size_hint (& self) -> http_body :: SizeHint { self . 0 . size_hint () } fn is_end_stream (& self) -> bool { self . 0 . is_end_stream () } }
};
}
