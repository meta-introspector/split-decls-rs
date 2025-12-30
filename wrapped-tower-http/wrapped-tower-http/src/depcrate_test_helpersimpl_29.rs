// Generated macro for impl_29 (impl)
macro_rules! Depcrate_test_helpersimpl_29 {
() => {
// Module: crate::test_helpers
// Provides: {"impl_29"}
// Dependencies: {}
impl < S > http_body :: Body for StreamBody < S > where S : TryStream , S :: Ok : Into < Bytes > , S :: Error : Into < BoxError > , { type Data = Bytes ; type Error = BoxError ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { let stream = self . project () . stream . get_pin_mut () ; match std :: task :: ready ! (stream . try_poll_next (cx)) { Some (Ok (chunk)) => Poll :: Ready (Some (Ok (Frame :: data (chunk . into ())))) , Some (Err (err)) => Poll :: Ready (Some (Err (err . into ()))) , None => Poll :: Ready (None) , } } }
};
}
