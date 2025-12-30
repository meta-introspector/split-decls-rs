// Generated macro for impl_805 (impl)
macro_rules! Depcrate_cors_allow_originimpl_805 {
() => {
// Module: crate::cors::allow_origin
// Provides: {"impl_805"}
// Dependencies: {}
impl AllowOriginFuture { fn ok (res : Option < (HeaderName , HeaderValue) >) -> Self { Self :: Ok { res } } fn fut < F : Future < Output = Option < (HeaderName , HeaderValue) > > + Send + 'static > (future : F ,) -> Self { Self :: Future { future : Box :: pin (future) , } } }
};
}
