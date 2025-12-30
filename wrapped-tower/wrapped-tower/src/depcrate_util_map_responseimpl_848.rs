// Generated macro for impl_848 (impl)
macro_rules! Depcrate_util_map_responseimpl_848 {
() => {
// Module: crate::util::map_response
// Provides: {"impl_848"}
// Dependencies: {}
impl < S , F , Request , Response > Service < Request > for MapResponse < S , F > where S : Service < Request > , F : FnOnce (S :: Response) -> Response + Clone , { type Response = Response ; type Error = S :: Error ; type Future = MapResponseFuture < S :: Future , F > ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } # [inline] fn call (& mut self , request : Request) -> Self :: Future { MapResponseFuture :: new (self . inner . call (request) . map_ok (self . f . clone ())) } }
};
}
