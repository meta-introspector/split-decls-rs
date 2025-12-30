// Generated macro for impl_1224 (impl)
macro_rules! Depcrate_services_fs_serve_dirimpl_1224 {
() => {
// Module: crate::services::fs::serve_dir
// Provides: {"impl_1224"}
// Dependencies: {}
impl < ReqBody > Service < Request < ReqBody > > for DefaultServeDirFallback where ReqBody : Send + 'static , { type Response = Response < ResponseBody > ; type Error = Infallible ; type Future = InfallibleResponseFuture < ReqBody , Self > ; fn poll_ready (& mut self , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { match self . 0 { } } fn call (& mut self , _req : Request < ReqBody >) -> Self :: Future { match self . 0 { } } }
};
}
