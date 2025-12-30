// Generated macro for macro_804 (macro)
macro_rules! Depcrate_cors_allow_originmacro_804 {
() => {
// Module: crate::cors::allow_origin
// Provides: {"macro_804"}
// Dependencies: {}
pin_project ! { # [project = AllowOriginFutureProj] pub (super) enum AllowOriginFuture { Ok { res : Option < (HeaderName , HeaderValue) > } , Future { # [pin] future : Pin < Box < dyn Future < Output = Option < (HeaderName , HeaderValue) >> + Send + 'static >> } , } }
};
}
