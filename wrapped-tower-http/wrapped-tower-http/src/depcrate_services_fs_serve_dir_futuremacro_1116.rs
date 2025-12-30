// Generated macro for macro_1116 (macro)
macro_rules! Depcrate_services_fs_serve_dir_futuremacro_1116 {
() => {
// Module: crate::services::fs::serve_dir::future
// Provides: {"macro_1116"}
// Dependencies: {}
pin_project ! { # [project = ResponseFutureInnerProj] pub (super) enum ResponseFutureInner < ReqBody , F > { OpenFileFuture { # [pin] future : BoxFuture <'static , io :: Result < OpenFileOutput >>, fallback_and_request : Option < (F , Request < ReqBody >) >, } , FallbackFuture { future : BoxFuture <'static , Result < Response < ResponseBody >, Infallible >>, } , InvalidPath { fallback_and_request : Option < (F , Request < ReqBody >) >, } , MethodNotAllowed , } }
};
}
