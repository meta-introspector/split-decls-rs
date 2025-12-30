// Generated macro for impl_32 (impl)
macro_rules! Depcrate_test_helpersimpl_32 {
() => {
// Module: crate::test_helpers
// Provides: {"impl_32"}
// Dependencies: {}
impl < B > http_body :: Body for WithTrailers < B > where B : http_body :: Body , { type Data = B :: Data ; type Error = B :: Error ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { let this = self . project () ; match std :: task :: ready ! (this . inner . poll_frame (cx)) { Some (frame) => Poll :: Ready (Some (frame)) , None => { if let Some (trailers) = this . trailers . take () { Poll :: Ready (Some (Ok (Frame :: trailers (trailers)))) } else { Poll :: Ready (None) } } } } }
};
}
