// Generated macro for c_buf_to_utf8_str (function)
macro_rules! Depcrate_unix_freebsd_utilsc_buf_to_utf8_str {
() => {
// Module: crate::unix::freebsd::utils
// Provides: {"c_buf_to_utf8_str"}
// Dependencies: {}
# [cfg (any (feature = "disk" , feature = "system" , feature = "network"))] pub (crate) fn c_buf_to_utf8_str (buf : & [libc :: c_char]) -> Option < & str > { unsafe { let buf : & [u8] = std :: slice :: from_raw_parts (buf . as_ptr () as _ , buf . len ()) ; std :: str :: from_utf8 (if let Some (pos) = buf . iter () . position (| x | * x == 0) { & buf [.. pos] } else { buf }) . ok () } }
};
}
