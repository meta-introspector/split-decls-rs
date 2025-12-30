// Generated macro for get_sys_value (function)
macro_rules! Depcrate_unix_freebsd_utilsget_sys_value {
() => {
// Module: crate::unix::freebsd::utils
// Provides: {"get_sys_value"}
// Dependencies: {}
# [cfg (any (feature = "system" , feature = "network"))] pub (crate) unsafe fn get_sys_value < T : Sized > (mib : & [libc :: c_int] , value : & mut T) -> bool { let mut len = std :: mem :: size_of :: < T > () as libc :: size_t ; unsafe { libc :: sysctl (mib . as_ptr () , mib . len () as _ , value as * mut _ as * mut _ , & mut len , std :: ptr :: null_mut () , 0 ,) == 0 } }
};
}
