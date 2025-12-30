// Generated macro for get_sys_value_str (function)
macro_rules! Depcrate_unix_freebsd_utilsget_sys_value_str {
() => {
// Module: crate::unix::freebsd::utils
// Provides: {"get_sys_value_str"}
// Dependencies: {}
# [cfg (feature = "system")] pub (crate) unsafe fn get_sys_value_str (mib : & [libc :: c_int] , buf : & mut [libc :: c_char] ,) -> Option < OsString > { let mut len = std :: mem :: size_of_val (buf) as libc :: size_t ; unsafe { if libc :: sysctl (mib . as_ptr () , mib . len () as _ , buf . as_mut_ptr () as * mut _ , & mut len , std :: ptr :: null_mut () , 0 ,) != 0 { return None ; } } Some (c_buf_to_os_string (& buf [.. len / std :: mem :: size_of :: < libc :: c_char > ()] ,)) }
};
}
