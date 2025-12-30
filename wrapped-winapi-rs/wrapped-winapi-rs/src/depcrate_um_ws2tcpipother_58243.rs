// Generated macro for other_58243 (other)
macro_rules! Depcrate_um_ws2tcpipother_58243 {
() => {
// Module: crate::um::ws2tcpip
// Provides: {"other_58243"}
// Dependencies: {}
extern "system" { pub fn inet_pton (Family : INT , pszAddrString : PCSTR , pAddrBuf : PVOID ,) -> INT ; pub fn InetPtonW (Family : INT , pszAddrString : PCWSTR , pAddrBuf : PVOID ,) -> INT ; pub fn inet_ntop (Family : INT , pAddr : * const VOID , pStringBuf : PSTR , StringBufSize : size_t ,) -> PCSTR ; pub fn InetNtopW (Family : INT , pAddr : * const VOID , pStringBuf : PWSTR , StringBufSize : size_t ,) -> PCWSTR ; }
};
}
