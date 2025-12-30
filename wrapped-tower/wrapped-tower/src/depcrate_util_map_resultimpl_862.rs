// Generated macro for impl_862 (impl)
macro_rules! Depcrate_util_map_resultimpl_862 {
() => {
// Module: crate::util::map_result
// Provides: {"impl_862"}
// Dependencies: {}
impl < S , F , Request , Response , Error > Service < Request > for MapResult < S , F > where S : Service < Request > , Error : From < S :: Error > , F : FnOnce (Result < S :: Response , S :: Error >) -> Result < Response , Error > + Clone , { type Response = Response ; type Error = Error ; type Future = MapResultFuture < S :: Future , F > ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) . map_err (Into :: into) } # [inline] fn call (& mut self , request : Request) -> Self :: Future { MapResultFuture :: new (self . inner . call (request) . map (self . f . clone ())) } }
};
}
