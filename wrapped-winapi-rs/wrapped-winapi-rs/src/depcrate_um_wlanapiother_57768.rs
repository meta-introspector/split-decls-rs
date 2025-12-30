// Generated macro for other_57768 (other)
macro_rules! Depcrate_um_wlanapiother_57768 {
() => {
// Module: crate::um::wlanapi
// Provides: {"other_57768"}
// Dependencies: {}
extern "system" { pub fn WFDStartOpenSession (hClientHandle : HANDLE , pDeviceAddress : PDOT11_MAC_ADDRESS , pvContext : PVOID , pfnCallback : WFD_OPEN_SESSION_COMPLETE_CALLBACK , phSessionHandle : PHANDLE ,) -> DWORD ; pub fn WFDCancelOpenSession (hSessionHandle : HANDLE ,) -> DWORD ; pub fn WFDOpenLegacySession (hClientHandle : HANDLE , pLegacyMacAddress : PDOT11_MAC_ADDRESS , phSessionHandle : * mut HANDLE , pGuidSessionInterface : * mut GUID ,) -> DWORD ; pub fn WFDCloseSession (hSessionHandle : HANDLE ,) -> DWORD ; pub fn WFDUpdateDeviceVisibility (pDeviceAddress : PDOT11_MAC_ADDRESS ,) -> DWORD ; }
};
}
