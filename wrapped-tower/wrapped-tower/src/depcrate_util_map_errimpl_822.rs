// Generated macro for impl_822 (impl)
macro_rules! Depcrate_util_map_errimpl_822 {
() => {
// Module: crate::util::map_err
// Provides: {"impl_822"}
// Dependencies: {}
impl < S , F , Request , Error > Service < Request > for MapErr < S , F > where S : Service < Request > , F : FnOnce (S :: Error) -> Error + Clone , { type Response = S :: Response ; type Error = Error ; type Future = MapErrFuture < S :: Future , F > ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) . map_err (self . f . clone ()) } # [inline] fn call (& mut self , request : Request) -> Self :: Future { MapErrFuture :: new (self . inner . call (request) . map_err (self . f . clone ())) } }
};
}
