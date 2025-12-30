// Generated macro for other_58230 (other)
macro_rules! Depcrate_um_ws2tcpipother_58230 {
() => {
// Module: crate::um::ws2tcpip
// Provides: {"other_58230"}
// Dependencies: {}
extern "system" { pub fn SetAddrInfoExA (pName : PCSTR , pServiceName : PCSTR , pAddresses : * mut SOCKET_ADDRESS , dwAddressCount : DWORD , lpBlob : LPBLOB , dwFlags : DWORD , dwNameSpace : DWORD , lpNspId : LPGUID , timeout : * mut timeval , lpOverlapped : LPOVERLAPPED , lpCompletionRoutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE , lpNameHandle : LPHANDLE ,) -> INT ; pub fn SetAddrInfoExW (pName : PCWSTR , pServiceName : PCWSTR , pAddresses : * mut SOCKET_ADDRESS , dwAddressCount : DWORD , lpBlob : LPBLOB , dwFlags : DWORD , dwNameSpace : DWORD , lpNspId : LPGUID , timeout : * mut timeval , lpOverlapped : LPOVERLAPPED , lpCompletionRoutine : LPLOOKUPSERVICE_COMPLETION_ROUTINE , lpNameHandle : LPHANDLE ,) -> INT ; }
};
}
