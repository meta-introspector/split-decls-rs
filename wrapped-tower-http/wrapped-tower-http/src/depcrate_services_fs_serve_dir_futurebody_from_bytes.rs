// Generated macro for body_from_bytes (function)
macro_rules! Depcrate_services_fs_serve_dir_futurebody_from_bytes {
() => {
// Module: crate::services::fs::serve_dir::future
// Provides: {"body_from_bytes"}
// Dependencies: {}
fn body_from_bytes (bytes : Bytes) -> ResponseBody { let body = Full :: from (bytes) . map_err (| err | match err { }) . boxed_unsync () ; ResponseBody :: new (UnsyncBoxBody :: new (body)) }
};
}
