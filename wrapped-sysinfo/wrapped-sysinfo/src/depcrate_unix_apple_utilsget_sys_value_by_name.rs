// Generated macro for get_sys_value_by_name (function)
macro_rules! Depcrate_unix_apple_utilsget_sys_value_by_name {
() => {
// Module: crate::unix::apple::utils
// Provides: {"get_sys_value_by_name"}
// Dependencies: {}
# [cfg (feature = "system")] pub (crate) unsafe fn get_sys_value_by_name (name : & [u8] , len : & mut usize , value : * mut libc :: c_void ,) -> bool { unsafe { libc :: sysctlbyname (name . as_ptr () as * const _ , value , len , std :: ptr :: null_mut () , 0 ,) == 0 } }
};
}
