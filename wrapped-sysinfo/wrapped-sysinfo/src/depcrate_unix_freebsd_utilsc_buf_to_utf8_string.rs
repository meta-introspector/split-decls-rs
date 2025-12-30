// Generated macro for c_buf_to_utf8_string (function)
macro_rules! Depcrate_unix_freebsd_utilsc_buf_to_utf8_string {
() => {
// Module: crate::unix::freebsd::utils
// Provides: {"c_buf_to_utf8_string"}
// Dependencies: {}
# [cfg (any (feature = "disk" , feature = "system" , feature = "network"))] pub (crate) fn c_buf_to_utf8_string (buf : & [libc :: c_char]) -> Option < String > { c_buf_to_utf8_str (buf) . map (| s | s . to_owned ()) }
};
}
