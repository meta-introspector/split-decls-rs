// Generated macro for get_sys_value_array (function)
macro_rules! Depcrate_unix_freebsd_utilsget_sys_value_array {
() => {
// Module: crate::unix::freebsd::utils
// Provides: {"get_sys_value_array"}
// Dependencies: {}
# [cfg (feature = "system")] pub (crate) unsafe fn get_sys_value_array < T : Sized > (mib : & [libc :: c_int] , value : & mut [T]) -> bool { let mut len = std :: mem :: size_of_val (value) as libc :: size_t ; unsafe { libc :: sysctl (mib . as_ptr () , mib . len () as _ , value . as_mut_ptr () as * mut _ , & mut len as * mut _ , std :: ptr :: null_mut () , 0 ,) == 0 } }
};
}
