// Generated macro for other_57702 (other)
macro_rules! Depcrate_um_wlanapiother_57702 {
() => {
// Module: crate::um::wlanapi
// Provides: {"other_57702"}
// Dependencies: {}
extern "system" { pub fn WlanUIEditProfile (dwClientVersion : DWORD , wstrProfileName : LPCWSTR , pInterfaceGuid : * mut GUID , hWnd : HWND , wlStartPage : WL_DISPLAY_PAGES , pReserved : PVOID , pWlanReasonCode : PWLAN_REASON_CODE ,) -> DWORD ; }
};
}
