// Generated macro for __isPlatformVersionAtLeast (function)
macro_rules! Depcrate_sys_platform_version_darwin_public_extern__isPlatformVersionAtLeast {
() => {
// Module: crate::sys::platform_version::darwin::public_extern
// Provides: {"__isPlatformVersionAtLeast"}
// Dependencies: {}
# [doc = " Whether the current platform's OS version is higher than or equal to the given version."] # [doc = ""] # [doc = " The first argument is the _base_ Mach-O platform (i.e. `PLATFORM_MACOS`, `PLATFORM_IOS`, etc.,"] # [doc = " but not `PLATFORM_IOSSIMULATOR` or `PLATFORM_MACCATALYST`) of the invoking binary."] # [doc = ""] # [doc = " Arguments are specified statically by Clang. Inlining with LTO should allow the versions to be"] # [doc = " combined into a single `u32`, which should make comparisons faster, and should make the"] # [doc = " `BASE_TARGET_PLATFORM` check a no-op."] # [rustc_std_internal_symbol] # [linkage = "weak"] pub (super) extern "C" fn __isPlatformVersionAtLeast (platform : i32 , major : i32 , minor : i32 , subminor : i32 ,) -> i32 { let version = pack_i32_os_version (major , minor , subminor) ; const BASE_TARGET_PLATFORM : i32 = if cfg ! (target_os = "macos") { 1 } else if cfg ! (target_os = "ios") { 2 } else if cfg ! (target_os = "tvos") { 3 } else if cfg ! (target_os = "watchos") { 4 } else if cfg ! (target_os = "visionos") { 11 } else { 0 } ; debug_assert_eq ! (platform , BASE_TARGET_PLATFORM , "invalid platform provided to __isPlatformVersionAtLeast" ,) ; (version <= current_version ()) as i32 }
};
}
