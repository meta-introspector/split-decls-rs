// Generated macro for invalid_plist (function)
macro_rules! Depcrate_sys_platform_version_darwin_testsinvalid_plist {
() => {
// Module: crate::sys::platform_version::darwin::tests
// Provides: {"invalid_plist"}
// Dependencies: {}
# [test] # [should_panic = "SystemVersion.plist did not contain a dictionary at the top level"] fn invalid_plist () { let cf_handle = CFHandle :: new () ; let _ = parse_version_from_plist (& cf_handle , b"INVALID") ; }
};
}
