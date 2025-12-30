// Generated macro for c_buf_to_os_str (function)
macro_rules! Depcrate_unix_freebsd_utilsc_buf_to_os_str {
() => {
// Module: crate::unix::freebsd::utils
// Provides: {"c_buf_to_os_str"}
// Dependencies: {}
# [cfg (feature = "system")] pub (crate) fn c_buf_to_os_str (buf : & [libc :: c_char]) -> & OsStr { unsafe { let buf : & [u8] = std :: slice :: from_raw_parts (buf . as_ptr () as _ , buf . len ()) ; OsStr :: from_bytes (if let Some (pos) = buf . iter () . position (| x | * x == 0) { & buf [.. pos] } else { buf }) } }
};
}
