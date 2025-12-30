// Generated macro for other_53313 (other)
macro_rules! Depcrate_um_winsock2other_53313 {
() => {
// Module: crate::um::winsock2
// Provides: {"other_53313"}
// Dependencies: {}
extern "system" { pub fn accept (s : SOCKET , addr : * mut SOCKADDR , addrlen : * mut c_int ,) -> SOCKET ; pub fn bind (s : SOCKET , name : * const SOCKADDR , namelen : c_int ,) -> c_int ; pub fn closesocket (s : SOCKET ,) -> c_int ; pub fn connect (s : SOCKET , name : * const SOCKADDR , namelen : c_int ,) -> c_int ; pub fn ioctlsocket (s : SOCKET , cmd : c_long , argp : * mut u_long ,) -> c_int ; pub fn getpeername (s : SOCKET , name : * mut SOCKADDR , namelen : * mut c_int ,) -> c_int ; pub fn getsockname (s : SOCKET , name : * mut SOCKADDR , namelen : * mut c_int ,) -> c_int ; pub fn getsockopt (s : SOCKET , level : c_int , optname : c_int , optval : * mut c_char , optlen : * mut c_int ,) -> c_int ; pub fn htonl (hostlong : u_long ,) -> u_long ; pub fn htons (hostshort : u_short ,) -> u_short ; pub fn inet_addr (cp : * const c_char ,) -> c_ulong ; pub fn inet_ntoa (_in : in_addr ,) -> * mut c_char ; }
};
}
