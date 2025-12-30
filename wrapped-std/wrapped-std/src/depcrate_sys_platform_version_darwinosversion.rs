// Generated macro for OSVersion (type)
macro_rules! Depcrate_sys_platform_version_darwinOSVersion {
() => {
// Module: crate::sys::platform_version::darwin
// Provides: {"OSVersion"}
// Dependencies: {}
# [doc = " The version of the operating system."] # [doc = ""] # [doc = " We use a packed u32 here to allow for fast comparisons and to match Mach-O's `LC_BUILD_VERSION`."] type OSVersion = u32 ;
};
}
