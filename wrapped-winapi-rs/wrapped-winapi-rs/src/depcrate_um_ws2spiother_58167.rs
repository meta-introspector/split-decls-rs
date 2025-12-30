// Generated macro for other_58167 (other)
macro_rules! Depcrate_um_ws2spiother_58167 {
() => {
// Module: crate::um::ws2spi
// Provides: {"other_58167"}
// Dependencies: {}
extern "system" { # [cfg (target_pointer_width = "64")] pub fn WSCEnableNSProvider32 (lpProviderId : LPGUID , fEnable : BOOL ,) -> INT ; # [cfg (target_pointer_width = "64")] pub fn WSCInstallProviderAndChains64_32 (lpProviderId : LPGUID , lpszProviderDllPath : LPWSTR , lpszProviderDllPath32 : LPWSTR , lpszLspName : LPWSTR , dwServiceFlags : DWORD , lpProtocolInfoList : LPWSAPROTOCOL_INFOW , dwNumberOfEntries : DWORD , lpdwCatalogEntryId : LPDWORD , lpErrno : LPINT ,) -> c_int ; # [cfg (target_pointer_width = "32")] pub fn WSCInstallProviderAndChains (lpProviderId : LPGUID , lpszProviderDllPath : LPWSTR , lpszLspName : LPWSTR , dwServiceFlags : DWORD , lpProtocolInfoList : LPWSAPROTOCOL_INFOW , dwNumberOfEntries : DWORD , lpdwCatalogEntryId : LPDWORD , lpErrno : LPINT ,) -> c_int ; }
};
}
