// Generated macro for macro_1114 (macro)
macro_rules! Depcrate_services_fs_serve_dir_futuremacro_1114 {
() => {
// Module: crate::services::fs::serve_dir::future
// Provides: {"macro_1114"}
// Dependencies: {}
pin_project ! { # [doc = " Response future of [`ServeDir::try_call()`][`super::ServeDir::try_call()`]."] pub struct ResponseFuture < ReqBody , F = DefaultServeDirFallback > { # [pin] pub (super) inner : ResponseFutureInner < ReqBody , F >, } }
};
}
