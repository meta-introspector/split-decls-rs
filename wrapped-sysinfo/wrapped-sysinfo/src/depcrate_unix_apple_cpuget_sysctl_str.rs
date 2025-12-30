// Generated macro for get_sysctl_str (function)
macro_rules! Depcrate_unix_apple_cpuget_sysctl_str {
() => {
// Module: crate::unix::apple::cpu
// Provides: {"get_sysctl_str"}
// Dependencies: {}
pub (crate) fn get_sysctl_str (s : & [u8]) -> String { let mut len = 0 ; unsafe { libc :: sysctlbyname (s . as_ptr () as * const c_char , std :: ptr :: null_mut () , & mut len , std :: ptr :: null_mut () , 0 ,) ; if len < 1 { return String :: new () ; } let mut buf = Vec :: with_capacity (len) ; libc :: sysctlbyname (s . as_ptr () as * const c_char , buf . as_mut_ptr () as _ , & mut len , std :: ptr :: null_mut () , 0 ,) ; if len > 0 { buf . set_len (len) ; while buf . last () == Some (& b'\0') { buf . pop () ; } String :: from_utf8 (buf) . unwrap_or_else (| _ | String :: new ()) } else { String :: new () } } }
};
}
