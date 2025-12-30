// Generated macro for impl_724 (impl)
macro_rules! Depcrate_util_boxed_unsyncimpl_724 {
() => {
// Module: crate::util::boxed::unsync
// Provides: {"impl_724"}
// Dependencies: {}
impl < T , U , E > Service < T > for UnsyncBoxService < T , U , E > { type Response = U ; type Error = E ; type Future = UnsyncBoxFuture < U , E > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , E > > { self . inner . poll_ready (cx) } fn call (& mut self , request : T) -> UnsyncBoxFuture < U , E > { self . inner . call (request) } }
};
}
