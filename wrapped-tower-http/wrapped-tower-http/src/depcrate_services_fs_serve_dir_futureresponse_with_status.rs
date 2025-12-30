// Generated macro for response_with_status (function)
macro_rules! Depcrate_services_fs_serve_dir_futureresponse_with_status {
() => {
// Module: crate::services::fs::serve_dir::future
// Provides: {"response_with_status"}
// Dependencies: {}
fn response_with_status (status : StatusCode) -> Response < ResponseBody > { Response :: builder () . status (status) . body (empty_body ()) . unwrap () }
};
}
