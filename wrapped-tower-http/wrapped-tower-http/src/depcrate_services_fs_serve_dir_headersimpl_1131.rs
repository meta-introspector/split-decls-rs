// Generated macro for impl_1131 (impl)
macro_rules! Depcrate_services_fs_serve_dir_headersimpl_1131 {
() => {
// Module: crate::services::fs::serve_dir::headers
// Provides: {"impl_1131"}
// Dependencies: {}
impl IfModifiedSince { # [doc = " Check if the supplied time means the resource has been modified."] pub (super) fn is_modified (& self , last_modified : & LastModified) -> bool { self . 0 < last_modified . 0 } # [doc = " convert a header value into a IfModifiedSince, invalid values are silentely ignored"] pub (super) fn from_header_value (value : & HeaderValue) -> Option < IfModifiedSince > { std :: str :: from_utf8 (value . as_bytes ()) . ok () . and_then (| value | httpdate :: parse_http_date (value) . ok ()) . map (| time | IfModifiedSince (time . into ())) } }
};
}
