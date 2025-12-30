// Generated macro for WSAGETASYNCERROR (function)
macro_rules! Depcrate_um_winsock2WSAGETASYNCERROR {
() => {
// Module: crate::um::winsock2
// Provides: {"WSAGETASYNCERROR"}
// Dependencies: {}
# [inline] pub fn WSAGETASYNCERROR (lParam : DWORD) -> WORD { HIWORD (lParam) }
};
}
