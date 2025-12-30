// Generated macro for impl_752 (impl)
macro_rules! Depcrate_metrics_in_flight_requestsimpl_752 {
() => {
// Module: crate::metrics::in_flight_requests
// Provides: {"impl_752"}
// Dependencies: {}
impl < B > Body for ResponseBody < B > where B : Body , { type Data = B :: Data ; type Error = B :: Error ; # [inline] fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < http_body :: Frame < Self :: Data > , Self :: Error > > > { self . project () . inner . poll_frame (cx) } # [inline] fn is_end_stream (& self) -> bool { self . inner . is_end_stream () } # [inline] fn size_hint (& self) -> http_body :: SizeHint { self . inner . size_hint () } }
};
}
