// Generated macro for impl_1240 (impl)
macro_rules! Depcrate_services_fsimpl_1240 {
() => {
// Module: crate::services::fs
// Provides: {"impl_1240"}
// Dependencies: {}
impl < T > Body for AsyncReadBody < T > where T : AsyncRead , { type Data = Bytes ; type Error = io :: Error ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { match std :: task :: ready ! (self . project () . reader . poll_next (cx)) { Some (Ok (chunk)) => Poll :: Ready (Some (Ok (Frame :: data (chunk)))) , Some (Err (err)) => Poll :: Ready (Some (Err (err))) , None => Poll :: Ready (None) , } } }
};
}
