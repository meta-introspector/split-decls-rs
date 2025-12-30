// Generated macro for pack_os_version (function)
macro_rules! Depcrate_sys_platform_version_darwinpack_os_version {
() => {
// Module: crate::sys::platform_version::darwin
// Provides: {"pack_os_version"}
// Dependencies: {}
# [doc = " Combine parts of a version into an [`OSVersion`]."] # [doc = ""] # [doc = " The size of the parts are inherently limited by Mach-O's `LC_BUILD_VERSION`."] # [inline] const fn pack_os_version (major : u16 , minor : u8 , patch : u8) -> OSVersion { let (major , minor , patch) = (major as u32 , minor as u32 , patch as u32) ; (major << 16) | (minor << 8) | patch }
};
}
