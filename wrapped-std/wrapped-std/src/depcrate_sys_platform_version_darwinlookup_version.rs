// Generated macro for lookup_version (function)
macro_rules! Depcrate_sys_platform_version_darwinlookup_version {
() => {
// Module: crate::sys::platform_version::darwin
// Provides: {"lookup_version"}
// Dependencies: {}
# [doc = " Look up the os version."] # [doc = ""] # [doc = " # Aborts"] # [doc = ""] # [doc = " Aborts if reading or parsing the version fails (or if the system was out of memory)."] # [doc = ""] # [doc = " We deliberately choose to abort, as having this silently return an invalid OS version would be"] # [doc = " impossible for a user to debug."] # [cold] extern "C" fn lookup_version () -> NonZero < OSVersion > { let version = version_from_sysctl () . unwrap_or_else (version_from_plist) ; NonZero :: new (version) . expect ("version cannot be 0.0.0") }
};
}
