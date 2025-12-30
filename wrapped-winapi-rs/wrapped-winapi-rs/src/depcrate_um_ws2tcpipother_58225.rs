// Generated macro for other_58225 (other)
macro_rules! Depcrate_um_ws2tcpipother_58225 {
() => {
// Module: crate::um::ws2tcpip
// Provides: {"other_58225"}
// Dependencies: {}
extern "system" { pub fn GetAddrInfoExA (pName : PCSTR , pServiceName : PCSTR , dwNameSpace : DWORD , lpNspId : LPGUID , hints : * const ADDRINFOEXA , ppResult : * mut PADDRINFOEXA , timeout : * mut timeval , lpOverlapped : LPOVERLAPPED , lpCompletionRoutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE , lpNameHandle : LPHANDLE ,) -> INT ; pub fn GetAddrInfoExW (pName : PCWSTR , pServiceName : PCWSTR , dwNameSpace : DWORD , lpNspId : LPGUID , hints : * const ADDRINFOEXW , ppResult : * mut PADDRINFOEXW , timeout : * mut timeval , lpOverlapped : LPOVERLAPPED , lpCompletionRoutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE , lpNameHandle : LPHANDLE ,) -> INT ; pub fn GetAddrInfoExCancel (lpHandle : LPHANDLE ,) -> INT ; pub fn GetAddrInfoExOverlappedResult (lpOverlapped : LPOVERLAPPED ,) -> INT ; }
};
}
