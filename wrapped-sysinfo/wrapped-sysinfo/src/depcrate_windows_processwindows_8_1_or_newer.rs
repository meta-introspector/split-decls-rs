// Generated macro for windows_8_1_or_newer (function)
macro_rules! Depcrate_windows_processwindows_8_1_or_newer {
() => {
// Module: crate::windows::process
// Provides: {"windows_8_1_or_newer"}
// Dependencies: {}
fn windows_8_1_or_newer () -> & 'static bool { static WINDOWS_8_1_OR_NEWER : OnceLock < bool > = OnceLock :: new () ; WINDOWS_8_1_OR_NEWER . get_or_init (| | unsafe { let mut version_info : OSVERSIONINFOEXW = MaybeUninit :: zeroed () . assume_init () ; version_info . dwOSVersionInfoSize = std :: mem :: size_of :: < OSVERSIONINFOEXW > () as u32 ; if RtlGetVersion ((& mut version_info as * mut OSVERSIONINFOEXW) . cast ()) . is_err () { return true ; } version_info . dwMajorVersion > 6 || version_info . dwMajorVersion == 6 && version_info . dwMinorVersion >= 3 }) }
};
}
