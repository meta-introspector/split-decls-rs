// Generated macro for pack_i32_os_version (function)
macro_rules! Depcrate_sys_platform_version_darwinpack_i32_os_version {
() => {
// Module: crate::sys::platform_version::darwin
// Provides: {"pack_i32_os_version"}
// Dependencies: {}
# [doc = " [`pack_os_version`], but takes `i32` and saturates."] # [doc = ""] # [doc = " Instead of using e.g. `major as u16`, which truncates."] # [inline] fn pack_i32_os_version (major : i32 , minor : i32 , patch : i32) -> OSVersion { let major : u16 = major . try_into () . unwrap_or (u16 :: MAX) ; let minor : u8 = minor . try_into () . unwrap_or (u8 :: MAX) ; let patch : u8 = patch . try_into () . unwrap_or (u8 :: MAX) ; pack_os_version (major , minor , patch) }
};
}
