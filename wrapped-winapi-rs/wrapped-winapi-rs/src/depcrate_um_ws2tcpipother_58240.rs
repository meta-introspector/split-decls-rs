// Generated macro for other_58240 (other)
macro_rules! Depcrate_um_ws2tcpipother_58240 {
() => {
// Module: crate::um::ws2tcpip
// Provides: {"other_58240"}
// Dependencies: {}
extern "system" { pub fn getnameinfo (pSockaddr : * const SOCKADDR , SockaddrLength : socklen_t , pNodeBuffer : PCHAR , NodeBufferSize : DWORD , pServiceBuffer : PCHAR , ServiceBufferSize : DWORD , Flags : INT ,) -> INT ; pub fn GetNameInfoW (pSockaddr : * const SOCKADDR , SockaddrLength : socklen_t , pNodeBuffer : PWCHAR , NodeBufferSize : DWORD , pServiceBuffer : PWCHAR , ServiceBufferSize : DWORD , Flags : INT ,) -> INT ; }
};
}
