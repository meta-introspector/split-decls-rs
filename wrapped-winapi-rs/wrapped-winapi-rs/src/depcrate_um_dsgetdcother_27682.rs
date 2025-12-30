// Generated macro for other_27682 (other)
macro_rules! Depcrate_um_dsgetdcother_27682 {
() => {
// Module: crate::um::dsgetdc
// Provides: {"other_27682"}
// Dependencies: {}
extern "system" { pub fn DsGetDcNameA (ComputerName : LPCSTR , DomainName : LPCSTR , DomainGuid : * mut GUID , SiteName : LPCSTR , Flags : ULONG , DomainControllerInfo : * mut PDOMAIN_CONTROLLER_INFOA ,) -> DWORD ; pub fn DsGetDcNameW (ComputerName : LPCWSTR , DomainName : LPCWSTR , DomainGuid : * mut GUID , SiteName : LPCWSTR , Flags : ULONG , DomainControllerInfo : * mut PDOMAIN_CONTROLLER_INFOW ,) -> DWORD ; pub fn DsGetSiteNameA (ComputerName : LPCSTR , SiteName : * mut LPSTR ,) -> DWORD ; pub fn DsGetSiteNameW (ComputerName : LPCWSTR , SiteName : * mut LPWSTR ,) -> DWORD ; pub fn DsValidateSubnetNameW (SubnetName : LPCWSTR ,) -> DWORD ; pub fn DsValidateSubnetNameA (SubnetName : LPCSTR ,) -> DWORD ; pub fn DsAddressToSiteNamesW (ComputerName : LPCWSTR , EntryCount : DWORD , SocketAddresses : PSOCKET_ADDRESS , SiteNames : * mut * mut LPWSTR ,) -> DWORD ; pub fn DsAddressToSiteNamesA (ComputerName : LPCSTR , EntryCount : DWORD , SocketAddresses : PSOCKET_ADDRESS , SiteNames : * mut * mut LPSTR ,) -> DWORD ; pub fn DsAddressToSiteNamesExW (ComputerName : LPCWSTR , EntryCount : DWORD , SocketAddresses : PSOCKET_ADDRESS , SiteNames : * mut * mut LPWSTR , SubnetNames : * mut * mut LPWSTR ,) -> DWORD ; pub fn DsAddressToSiteNamesExA (ComputerName : LPCSTR , EntryCount : DWORD , SocketAddresses : PSOCKET_ADDRESS , SiteNames : * mut * mut LPSTR , SubnetNames : * mut * mut LPSTR ,) -> DWORD ; }
};
}
