// Generated macro for h_errno (function)
macro_rules! Depcrate_um_winsock2h_errno {
() => {
// Module: crate::um::winsock2
// Provides: {"h_errno"}
// Dependencies: {}
# [inline] pub unsafe fn h_errno () -> c_int { WSAGetLastError () }
};
}
