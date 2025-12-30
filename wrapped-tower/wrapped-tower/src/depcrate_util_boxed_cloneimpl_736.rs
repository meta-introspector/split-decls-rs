// Generated macro for impl_736 (impl)
macro_rules! Depcrate_util_boxed_cloneimpl_736 {
() => {
// Module: crate::util::boxed_clone
// Provides: {"impl_736"}
// Dependencies: {}
impl < T , U , E > Service < T > for BoxCloneService < T , U , E > { type Response = U ; type Error = E ; type Future = BoxFuture < 'static , Result < U , E > > ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , E > > { self . 0 . poll_ready (cx) } # [inline] fn call (& mut self , request : T) -> Self :: Future { self . 0 . call (request) } }
};
}
