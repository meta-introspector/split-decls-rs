// Generated macro for c_buf_to_os_string (function)
macro_rules! Depcrate_unix_freebsd_utilsc_buf_to_os_string {
() => {
// Module: crate::unix::freebsd::utils
// Provides: {"c_buf_to_os_string"}
// Dependencies: {}
# [cfg (feature = "system")] pub (crate) fn c_buf_to_os_string (buf : & [libc :: c_char]) -> OsString { c_buf_to_os_str (buf) . to_owned () }
};
}
