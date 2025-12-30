// Generated macro for impl_1133 (impl)
macro_rules! Depcrate_services_fs_serve_dir_headersimpl_1133 {
() => {
// Module: crate::services::fs::serve_dir::headers
// Provides: {"impl_1133"}
// Dependencies: {}
impl IfUnmodifiedSince { # [doc = " Check if the supplied time passes the precondtion."] pub (super) fn precondition_passes (& self , last_modified : & LastModified) -> bool { self . 0 >= last_modified . 0 } # [doc = " Convert a header value into a IfModifiedSince, invalid values are silentely ignored"] pub (super) fn from_header_value (value : & HeaderValue) -> Option < IfUnmodifiedSince > { std :: str :: from_utf8 (value . as_bytes ()) . ok () . and_then (| value | httpdate :: parse_http_date (value) . ok ()) . map (| time | IfUnmodifiedSince (time . into ())) } }
};
}
