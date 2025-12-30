// Generated macro for WSAGETASYNCBUFLEN (function)
macro_rules! Depcrate_um_winsock2WSAGETASYNCBUFLEN {
() => {
// Module: crate::um::winsock2
// Provides: {"WSAGETASYNCBUFLEN"}
// Dependencies: {}
# [inline] pub fn WSAGETASYNCBUFLEN (lParam : DWORD) -> WORD { LOWORD (lParam) }
};
}
