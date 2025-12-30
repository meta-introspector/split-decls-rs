// Generated macro for impl_940 (impl)
macro_rules! Depcrate_util_thenimpl_940 {
() => {
// Module: crate::util::then
// Provides: {"impl_940"}
// Dependencies: {}
impl < S , F , Request , Response , Error , Fut > Service < Request > for Then < S , F > where S : Service < Request > , S :: Error : Into < Error > , F : FnOnce (Result < S :: Response , S :: Error >) -> Fut + Clone , Fut : Future < Output = Result < Response , Error > > , { type Response = Response ; type Error = Error ; type Future = ThenFuture < S :: Future , Fut , F > ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) . map_err (Into :: into) } # [inline] fn call (& mut self , request : Request) -> Self :: Future { ThenFuture :: new (self . inner . call (request) . then (self . f . clone ())) } }
};
}
