// Generated macro for impl_11 (impl)
macro_rules! Depcrate_bodytimpl_11 {
() => {
// Module: crate::bodyt
// Provides: {"impl_11"}
// Dependencies: {}
impl http_body :: Body for Body { type Data = Bytes ; type Error = crate :: Error ; fn poll_frame (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { Pin :: new (& mut self . 0) . poll_frame (cx) } fn is_end_stream (& self) -> bool { self . 0 . is_end_stream () } fn size_hint (& self) -> http_body :: SizeHint { self . 0 . size_hint () } }
};
}
