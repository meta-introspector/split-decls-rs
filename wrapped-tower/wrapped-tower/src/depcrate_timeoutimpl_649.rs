// Generated macro for impl_649 (impl)
macro_rules! Depcrate_timeoutimpl_649 {
() => {
// Module: crate::timeout
// Provides: {"impl_649"}
// Dependencies: {}
impl < S , Request > Service < Request > for Timeout < S > where S : Service < Request > , S :: Error : Into < crate :: BoxError > , { type Response = S :: Response ; type Error = crate :: BoxError ; type Future = ResponseFuture < S :: Future > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { match self . inner . poll_ready (cx) { Poll :: Pending => Poll :: Pending , Poll :: Ready (r) => Poll :: Ready (r . map_err (Into :: into)) , } } fn call (& mut self , request : Request) -> Self :: Future { let response = self . inner . call (request) ; let sleep = tokio :: time :: sleep (self . timeout) ; ResponseFuture :: new (response , sleep) } }
};
}
