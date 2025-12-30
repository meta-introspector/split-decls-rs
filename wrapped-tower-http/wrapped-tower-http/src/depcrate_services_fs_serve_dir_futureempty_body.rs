// Generated macro for empty_body (function)
macro_rules! Depcrate_services_fs_serve_dir_futureempty_body {
() => {
// Module: crate::services::fs::serve_dir::future
// Provides: {"empty_body"}
// Dependencies: {}
fn empty_body () -> ResponseBody { let body = Empty :: new () . map_err (| err | match err { }) . boxed_unsync () ; ResponseBody :: new (UnsyncBoxBody :: new (body)) }
};
}
