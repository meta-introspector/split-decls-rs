// Generated macro for impl_726 (impl)
macro_rules! Depcrate_util_boxed_unsyncimpl_726 {
() => {
// Module: crate::util::boxed::unsync
// Provides: {"impl_726"}
// Dependencies: {}
impl < S , Request > Service < Request > for UnsyncBoxed < S > where S : Service < Request > + 'static , S :: Future : 'static , { type Response = S :: Response ; type Error = S :: Error ; type Future = Pin < Box < dyn Future < Output = Result < S :: Response , S :: Error > > > > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . inner . poll_ready (cx) } fn call (& mut self , request : Request) -> Self :: Future { Box :: pin (self . inner . call (request)) } }
};
}
