// Generated macro for get_sys_value (function)
macro_rules! Depcrate_unix_apple_utilsget_sys_value {
() => {
// Module: crate::unix::apple::utils
// Provides: {"get_sys_value"}
// Dependencies: {}
# [cfg (feature = "system")] pub (crate) unsafe fn get_sys_value (mut len : usize , value : * mut libc :: c_void , mib : & mut [i32] ,) -> bool { unsafe { libc :: sysctl (mib . as_mut_ptr () , mib . len () as _ , value , & mut len as * mut _ , std :: ptr :: null_mut () , 0 ,) == 0 } }
};
}
