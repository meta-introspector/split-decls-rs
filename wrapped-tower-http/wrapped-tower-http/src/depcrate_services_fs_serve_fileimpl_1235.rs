// Generated macro for impl_1235 (impl)
macro_rules! Depcrate_services_fs_serve_fileimpl_1235 {
() => {
// Module: crate::services::fs::serve_file
// Provides: {"impl_1235"}
// Dependencies: {}
impl < ReqBody > Service < Request < ReqBody > > for ServeFile where ReqBody : Send + 'static , { type Error = < ServeDir as Service < Request < ReqBody > > > :: Error ; type Response = < ServeDir as Service < Request < ReqBody > > > :: Response ; type Future = < ServeDir as Service < Request < ReqBody > > > :: Future ; # [inline] fn poll_ready (& mut self , _cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } # [inline] fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { self . 0 . call (req) } }
};
}
