// Generated macro for impl_806 (impl)
macro_rules! Depcrate_cors_allow_originimpl_806 {
() => {
// Module: crate::cors::allow_origin
// Provides: {"impl_806"}
// Dependencies: {}
impl Future for AllowOriginFuture { type Output = Option < (HeaderName , HeaderValue) > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . project () { AllowOriginFutureProj :: Ok { res } => Poll :: Ready (res . take ()) , AllowOriginFutureProj :: Future { future } => future . poll (cx) , } } }
};
}
