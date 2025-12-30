// Generated macro for other_58143 (other)
macro_rules! Depcrate_um_ws2spiother_58143 {
() => {
// Module: crate::um::ws2spi
// Provides: {"other_58143"}
// Dependencies: {}
extern "system" { # [cfg (target_pointer_width = "64")] pub fn WSCDeinstallProvider32 (lpProviderId : LPGUID , lpErrno : LPINT ,) -> c_int ; pub fn WSCInstallProvider (lpProviderId : LPGUID , lpszProviderDllPath : * const WCHAR , lpProtocolInfoList : LPWSAPROTOCOL_INFOW , dwNumberOfEntries : DWORD , lpErrno : LPINT ,) -> c_int ; }
};
}
