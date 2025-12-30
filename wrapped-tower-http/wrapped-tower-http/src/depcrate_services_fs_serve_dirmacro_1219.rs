// Generated macro for macro_1219 (macro)
macro_rules! Depcrate_services_fs_serve_dirmacro_1219 {
() => {
// Module: crate::services::fs::serve_dir
// Provides: {"macro_1219"}
// Dependencies: {}
opaque_future ! { # [doc = " Response future of [`ServeDir`]."] pub type InfallibleResponseFuture < ReqBody , F > = futures_util :: future :: Map < ResponseFuture < ReqBody , F >, fn (Result < Response < ResponseBody >, io :: Error >) -> Result < Response < ResponseBody >, Infallible >, >; }
};
}
