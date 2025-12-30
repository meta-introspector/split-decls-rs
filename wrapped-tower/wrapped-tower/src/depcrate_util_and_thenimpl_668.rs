// Generated macro for impl_668 (impl)
macro_rules! Depcrate_util_and_thenimpl_668 {
() => {
// Module: crate::util::and_then
// Provides: {"impl_668"}
// Dependencies: {}
impl < S , F , Request , Fut > Service < Request > for AndThen < S , F > where S : Service < Request > , S :: Error : Into < Fut :: Error > , F : FnOnce (S :: Response) -> Fut + Clone , Fut : TryFuture , { type Response = Fut :: Ok ; type Error = Fut :: Error ; type Future = AndThenFuture < S :: Future , Fut , F > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) . map_err (Into :: into) } fn call (& mut self , request : Request) -> Self :: Future { AndThenFuture :: new (self . inner . call (request) . err_into () . and_then (self . f . clone ())) } }
};
}
