// Generated macro for other_27694 (other)
macro_rules! Depcrate_um_dsgetdcother_27694 {
() => {
// Module: crate::um::dsgetdc
// Provides: {"other_27694"}
// Dependencies: {}
extern "system" { pub fn DsEnumerateDomainTrustsW (ServerName : LPWSTR , Flags : ULONG , Domains : * mut PDS_DOMAIN_TRUSTSW , DomainCount : PULONG ,) -> DWORD ; pub fn DsEnumerateDomainTrustsA (ServerName : LPSTR , Flags : ULONG , Domains : * mut PDS_DOMAIN_TRUSTSA , DomainCount : PULONG ,) -> DWORD ; pub fn DsGetForestTrustInformationW (ServerName : LPCWSTR , TrustedDomainName : LPCWSTR , Flags : DWORD , ForestTrustInfo : * mut PLSA_FOREST_TRUST_INFORMATION ,) -> DWORD ; pub fn DsMergeForestTrustInformationW (DomainName : LPCWSTR , NewForestTrustInfo : PLSA_FOREST_TRUST_INFORMATION , OldForestTrustInfo : PLSA_FOREST_TRUST_INFORMATION , MergedForestTrustInfo : * mut PLSA_FOREST_TRUST_INFORMATION ,) -> DWORD ; pub fn DsGetDcSiteCoverageW (ServerName : LPCWSTR , EntryCount : PULONG , SiteNames : * mut * mut LPWSTR ,) -> DWORD ; pub fn DsGetDcSiteCoverageA (ServerName : LPCSTR , EntryCount : PULONG , SiteNames : * mut * mut LPSTR ,) -> DWORD ; pub fn DsDeregisterDnsHostRecordsW (ServerName : LPWSTR , DnsDomainName : LPWSTR , DomainGuid : * mut GUID , DsaGuid : * mut GUID , DnsHostName : LPWSTR ,) -> DWORD ; pub fn DsDeregisterDnsHostRecordsA (ServerName : LPSTR , DnsDomainName : LPSTR , DomainGuid : * mut GUID , DsaGuid : * mut GUID , DnsHostName : LPSTR ,) -> DWORD ; }
};
}
