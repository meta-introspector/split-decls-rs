// Generated macro for redirect_stderr (function)
macro_rules! Depcrate_commandsredirect_stderr {
() => {
// Module: crate::commands
// Provides: {"redirect_stderr"}
// Dependencies: {}
# [cfg (windows)] fn redirect_stderr (f : File) { use std :: os :: windows :: io :: IntoRawHandle ; use windows_sys :: Win32 :: System :: Console :: { STD_ERROR_HANDLE , SetStdHandle } ; unsafe { SetStdHandle (STD_ERROR_HANDLE , f . into_raw_handle () as _) ; } }
};
}
