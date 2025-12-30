// Generated macro for impl_1115 (impl)
macro_rules! Depcrate_services_fs_serve_dir_futureimpl_1115 {
() => {
// Module: crate::services::fs::serve_dir::future
// Provides: {"impl_1115"}
// Dependencies: {}
impl < ReqBody , F > ResponseFuture < ReqBody , F > { pub (super) fn open_file_future (future : BoxFuture < 'static , io :: Result < OpenFileOutput > > , fallback_and_request : Option < (F , Request < ReqBody >) > ,) -> Self { Self { inner : ResponseFutureInner :: OpenFileFuture { future , fallback_and_request , } , } } pub (super) fn invalid_path (fallback_and_request : Option < (F , Request < ReqBody >) >) -> Self { Self { inner : ResponseFutureInner :: InvalidPath { fallback_and_request , } , } } pub (super) fn method_not_allowed () -> Self { Self { inner : ResponseFutureInner :: MethodNotAllowed , } } }
};
}
