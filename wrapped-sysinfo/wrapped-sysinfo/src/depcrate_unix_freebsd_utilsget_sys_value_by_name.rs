// Generated macro for get_sys_value_by_name (function)
macro_rules! Depcrate_unix_freebsd_utilsget_sys_value_by_name {
() => {
// Module: crate::unix::freebsd::utils
// Provides: {"get_sys_value_by_name"}
// Dependencies: {}
# [cfg (any (feature = "system" , feature = "component"))] pub (crate) unsafe fn get_sys_value_by_name < T : Sized > (name : & [u8] , value : & mut T) -> bool { let mut len = std :: mem :: size_of :: < T > () as libc :: size_t ; let original = len ; unsafe { libc :: sysctlbyname (name . as_ptr () as * const libc :: c_char , value as * mut _ as * mut _ , & mut len , std :: ptr :: null_mut () , 0 ,) == 0 && original == len } }
};
}
