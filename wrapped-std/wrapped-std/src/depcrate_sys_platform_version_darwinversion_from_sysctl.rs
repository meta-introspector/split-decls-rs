// Generated macro for version_from_sysctl (function)
macro_rules! Depcrate_sys_platform_version_darwinversion_from_sysctl {
() => {
// Module: crate::sys::platform_version::darwin
// Provides: {"version_from_sysctl"}
// Dependencies: {}
# [doc = " Read the version from `kern.osproductversion` or `kern.iossupportversion`."] # [doc = ""] # [doc = " This is faster than `version_from_plist`, since it doesn't need to invoke `dlsym`."] fn version_from_sysctl () -> Option < OSVersion > { if cfg ! (target_abi = "sim") { return None ; } let sysctl_version = | name : & CStr | { let mut buf : [u8 ; 32] = [0 ; 32] ; let mut size = buf . len () ; let ptr = buf . as_mut_ptr () . cast () ; let ret = unsafe { libc :: sysctlbyname (name . as_ptr () , ptr , & mut size , null_mut () , 0) } ; if ret != 0 { return None ; } let buf = & buf [.. (size - 1)] ; if buf . is_empty () { return None ; } Some (parse_os_version (buf) . unwrap_or_else (| err | { panic ! ("failed parsing version from sysctl ({}): {err}" , ByteStr :: new (buf)) })) } ; if cfg ! (target_os = "ios") { if let Some (ios_support_version) = sysctl_version (c"kern.iossupportversion") { return Some (ios_support_version) ; } if cfg ! (target_abi = "macabi") { return None ; } } sysctl_version (c"kern.osproductversion") }
};
}
