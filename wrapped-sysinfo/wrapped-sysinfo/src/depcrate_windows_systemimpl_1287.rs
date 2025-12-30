// Generated macro for impl_1287 (impl)
macro_rules! Depcrate_windows_systemimpl_1287 {
() => {
// Module: crate::windows::system
// Provides: {"impl_1287"}
// Dependencies: {}
impl SystemInner { fn is_windows_eleven () -> bool { WINDOWS_ELEVEN_BUILD_NUMBER <= Self :: kernel_version () . unwrap_or_default () . parse () . unwrap_or (0) } }
};
}
