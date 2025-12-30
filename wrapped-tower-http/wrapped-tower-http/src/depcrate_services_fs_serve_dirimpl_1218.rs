// Generated macro for impl_1218 (impl)
macro_rules! Depcrate_services_fs_serve_dirimpl_1218 {
() => {
// Module: crate::services::fs::serve_dir
// Provides: {"impl_1218"}
// Dependencies: {}
impl < ReqBody , F , FResBody > Service < Request < ReqBody > > for ServeDir < F > where F : Service < Request < ReqBody > , Response = Response < FResBody > , Error = Infallible > + Clone , F :: Future : Send + 'static , FResBody : http_body :: Body < Data = Bytes > + Send + 'static , FResBody :: Error : Into < Box < dyn std :: error :: Error + Send + Sync > > , { type Response = Response < ResponseBody > ; type Error = Infallible ; type Future = InfallibleResponseFuture < ReqBody , F > ; # [inline] fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { if let Some (fallback) = & mut self . fallback { fallback . poll_ready (cx) } else { Poll :: Ready (Ok (())) } } fn call (& mut self , req : Request < ReqBody >) -> Self :: Future { let future = self . try_call (req) . map (| result : Result < _ , _ > | -> Result < _ , Infallible > { let response = result . unwrap_or_else (| err | { tracing :: error ! (error = % err , "Failed to read file") ; let body = ResponseBody :: new (UnsyncBoxBody :: new (Empty :: new () . map_err (| err | match err { }) . boxed_unsync () ,)) ; Response :: builder () . status (StatusCode :: INTERNAL_SERVER_ERROR) . body (body) . unwrap () }) ; Ok (response) } as _) ; InfallibleResponseFuture :: new (future) } }
};
}
