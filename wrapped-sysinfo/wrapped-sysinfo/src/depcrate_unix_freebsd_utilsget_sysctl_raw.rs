// Generated macro for get_sysctl_raw (function)
macro_rules! Depcrate_unix_freebsd_utilsget_sysctl_raw {
() => {
// Module: crate::unix::freebsd::utils
// Provides: {"get_sysctl_raw"}
// Dependencies: {}
# [cfg (feature = "system")] pub (crate) unsafe fn get_sysctl_raw (mib : & [libc :: c_int] , data : * mut () , len : & mut libc :: size_t ,) -> Option < () > { unsafe { if libc :: sysctl (mib . as_ptr () , mib . len () as _ , data as _ , len , std :: ptr :: null_mut () , 0 ,) != 0 { None } else { Some (()) } } }
};
}
