// Generated macro for is_server (function)
macro_rules! Depcrateis_server {
() => {
// Module: crate
// Provides: {"is_server"}
// Dependencies: {}
# [doc = " Determines if the currently running operating system is a Windows Server release."] pub fn is_server () -> bool { let mut info = OSVERSIONINFOEXW :: new () ; unsafe { RtlGetVersion (& mut info as * mut _ as * mut _) ; } info . wProductType as u32 != VER_NT_WORKSTATION }
};
}
