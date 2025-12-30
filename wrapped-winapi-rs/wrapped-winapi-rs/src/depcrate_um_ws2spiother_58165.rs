// Generated macro for other_58165 (other)
macro_rules! Depcrate_um_ws2spiother_58165 {
() => {
// Module: crate::um::ws2spi
// Provides: {"other_58165"}
// Dependencies: {}
extern "system" { pub fn WSCInstallNameSpaceEx (lpszIdentifier : LPWSTR , lpszPathName : LPWSTR , dwNameSpace : DWORD , dwVersion : DWORD , lpProviderId : LPGUID , lpProviderSpecific : LPBLOB ,) -> INT ; # [cfg (target_pointer_width = "64")] pub fn WSCInstallNameSpaceEx32 (lpszIdentifier : LPWSTR , lpszPathName : LPWSTR , dwNameSpace : DWORD , dwVersion : DWORD , lpProviderId : LPGUID , lpProviderSpecific : LPBLOB ,) -> INT ; # [cfg (target_pointer_width = "64")] pub fn WSCUnInstallNameSpace32 (lpProviderId : LPGUID ,) -> INT ; pub fn WSCEnableNSProvider (lpProviderId : LPGUID , fEnable : BOOL ,) -> INT ; }
};
}
