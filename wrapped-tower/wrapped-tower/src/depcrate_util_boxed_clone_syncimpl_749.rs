// Generated macro for impl_749 (impl)
macro_rules! Depcrate_util_boxed_clone_syncimpl_749 {
() => {
// Module: crate::util::boxed_clone_sync
// Provides: {"impl_749"}
// Dependencies: {}
impl < T , U , E > Service < T > for BoxCloneSyncService < T , U , E > { type Response = U ; type Error = E ; type Future = BoxFuture < 'static , Result < U , E > > ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , E > > { self . 0 . poll_ready (cx) } # [inline] fn call (& mut self , request : T) -> Self :: Future { self . 0 . call (request) } }
};
}
