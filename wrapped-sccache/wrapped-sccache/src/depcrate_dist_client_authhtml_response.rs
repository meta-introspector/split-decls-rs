// Generated macro for html_response (function)
macro_rules! Depcrate_dist_client_authhtml_response {
() => {
// Module: crate::dist::client_auth
// Provides: {"html_response"}
// Dependencies: {}
fn html_response (body : & 'static str) -> Response < Full < Bytes > > { Response :: builder () . header (CONTENT_TYPE , mime :: TEXT_HTML . to_string ()) . header (CONTENT_LENGTH , body . len ()) . body (body . into ()) . unwrap () }
};
}
