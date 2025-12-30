// Generated macro for enabled_pointer_authentication_keys (function)
macro_rules! Depcrate_process_prctlenabled_pointer_authentication_keys {
() => {
// Module: crate::process::prctl
// Provides: {"enabled_pointer_authentication_keys"}
// Dependencies: {}
# [doc = " Get enabled pointer authentication keys."] # [doc = ""] # [doc = " # References"] # [doc = "  - [`prctl(PR_PAC_GET_ENABLED_KEYS,…)`]"] # [doc = ""] # [doc = " [`prctl(PR_PAC_GET_ENABLED_KEYS,…)`]: https://www.kernel.org/doc/html/v6.13/arch/arm64/pointer-authentication.html"] # [inline] # [doc (alias = "PR_PAC_GET_ENABLED_KEYS")] # [cfg (linux_raw_dep)] pub fn enabled_pointer_authentication_keys () -> io :: Result < PointerAuthenticationKeys > { let r = unsafe { prctl_1arg (PR_PAC_GET_ENABLED_KEYS) ? } as c_uint ; PointerAuthenticationKeys :: from_bits (r) . ok_or (io :: Errno :: RANGE) }
};
}
