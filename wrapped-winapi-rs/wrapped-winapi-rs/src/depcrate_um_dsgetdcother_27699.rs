// Generated macro for other_27699 (other)
macro_rules! Depcrate_um_dsgetdcother_27699 {
() => {
// Module: crate::um::dsgetdc
// Provides: {"other_27699"}
// Dependencies: {}
extern "system" { pub fn DsGetDcOpenW (DnsName : LPCWSTR , OptionFlags : ULONG , SiteName : LPCWSTR , DomainGuid : * mut GUID , DnsForestName : LPCWSTR , DcFlags : ULONG , RetGetDcContext : PHANDLE ,) -> DWORD ; pub fn DsGetDcOpenA (DnsName : LPCSTR , OptionFlags : ULONG , SiteName : LPCSTR , DomainGuid : * mut GUID , DnsForestName : LPCSTR , DcFlags : ULONG , RetGetDcContext : PHANDLE ,) -> DWORD ; pub fn DsGetDcNextA (GetDcContextHandle : HANDLE , SockAddressCount : PULONG , SockAddresses : * mut LPSOCKET_ADDRESS , DnsHostName : * mut LPSTR ,) -> DWORD ; pub fn DsGetDcNextW (GetDcContextHandle : HANDLE , SockAddressCount : PULONG , SockAddresses : * mut LPSOCKET_ADDRESS , DnsHostName : * mut LPWSTR ,) -> DWORD ; pub fn DsGetDcCloseW (GetDcContextHandle : HANDLE ,) ; }
};
}
