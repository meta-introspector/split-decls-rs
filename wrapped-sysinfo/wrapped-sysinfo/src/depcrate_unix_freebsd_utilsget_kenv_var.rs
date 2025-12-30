// Generated macro for get_kenv_var (function)
macro_rules! Depcrate_unix_freebsd_utilsget_kenv_var {
() => {
// Module: crate::unix::freebsd::utils
// Provides: {"get_kenv_var"}
// Dependencies: {}
# [cfg (feature = "system")] pub (crate) fn get_kenv_var (name : & [u8]) -> Option < String > { let mut buf : [libc :: c_char ; libc :: KENV_MVALLEN as usize] = [0 ; libc :: KENV_MVALLEN as usize] ; let size = unsafe { libc :: kenv (libc :: KENV_GET as _ , name . as_ptr () as _ , buf . as_mut_ptr () as _ , buf . len () as _ ,) as isize } ; if size < 0 { return None ; } c_buf_to_utf8_string (& buf [.. size as usize]) }
};
}
