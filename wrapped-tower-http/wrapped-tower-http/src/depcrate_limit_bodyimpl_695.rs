// Generated macro for impl_695 (impl)
macro_rules! Depcrate_limit_bodyimpl_695 {
() => {
// Module: crate::limit::body
// Provides: {"impl_695"}
// Dependencies: {}
impl < B > Body for ResponseBody < B > where B : Body < Data = Bytes > , { type Data = Bytes ; type Error = B :: Error ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < http_body :: Frame < Self :: Data > , Self :: Error > > > { match self . project () . inner . project () { BodyProj :: PayloadTooLarge { body } => body . poll_frame (cx) . map_err (| err | match err { }) , BodyProj :: Body { body } => body . poll_frame (cx) , } } fn is_end_stream (& self) -> bool { match & self . inner { ResponseBodyInner :: PayloadTooLarge { body } => body . is_end_stream () , ResponseBodyInner :: Body { body } => body . is_end_stream () , } } fn size_hint (& self) -> SizeHint { match & self . inner { ResponseBodyInner :: PayloadTooLarge { body } => body . size_hint () , ResponseBodyInner :: Body { body } => body . size_hint () , } } }
};
}
