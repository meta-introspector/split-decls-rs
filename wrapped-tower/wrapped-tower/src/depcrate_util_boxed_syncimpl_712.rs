// Generated macro for impl_712 (impl)
macro_rules! Depcrate_util_boxed_syncimpl_712 {
() => {
// Module: crate::util::boxed::sync
// Provides: {"impl_712"}
// Dependencies: {}
impl < T , U , E > Service < T > for BoxService < T , U , E > { type Response = U ; type Error = E ; type Future = BoxFuture < U , E > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , E > > { self . inner . get_mut () . poll_ready (cx) } fn call (& mut self , request : T) -> BoxFuture < U , E > { self . inner . get_mut () . call (request) } }
};
}
