// Generated macro for get_sys_value_str_by_name (function)
macro_rules! Depcrate_unix_freebsd_utilsget_sys_value_str_by_name {
() => {
// Module: crate::unix::freebsd::utils
// Provides: {"get_sys_value_str_by_name"}
// Dependencies: {}
# [cfg (any (feature = "system" , feature = "disk"))] pub (crate) fn get_sys_value_str_by_name (name : & [u8]) -> Option < String > { let mut size = 0 ; unsafe { if libc :: sysctlbyname (name . as_ptr () as * const libc :: c_char , std :: ptr :: null_mut () , & mut size , std :: ptr :: null_mut () , 0 ,) == 0 && size > 0 { let mut buf : Vec < libc :: c_char > = vec ! [0 ; size as _] ; if libc :: sysctlbyname (name . as_ptr () as * const libc :: c_char , buf . as_mut_ptr () as * mut _ , & mut size , std :: ptr :: null_mut () , 0 ,) == 0 && size > 0 { c_buf_to_utf8_string (& buf) } else { None } } else { None } } }
};
}
