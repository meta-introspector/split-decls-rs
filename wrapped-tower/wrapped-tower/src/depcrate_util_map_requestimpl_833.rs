// Generated macro for impl_833 (impl)
macro_rules! Depcrate_util_map_requestimpl_833 {
() => {
// Module: crate::util::map_request
// Provides: {"impl_833"}
// Dependencies: {}
impl < S , F , R1 , R2 > Service < R1 > for MapRequest < S , F > where S : Service < R2 > , F : FnMut (R1) -> R2 , { type Response = S :: Response ; type Error = S :: Error ; type Future = S :: Future ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , S :: Error > > { self . inner . poll_ready (cx) } # [inline] fn call (& mut self , request : R1) -> S :: Future { self . inner . call ((self . f) (request)) } }
};
}
