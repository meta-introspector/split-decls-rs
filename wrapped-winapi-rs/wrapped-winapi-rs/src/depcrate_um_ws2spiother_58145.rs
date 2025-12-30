// Generated macro for other_58145 (other)
macro_rules! Depcrate_um_ws2spiother_58145 {
() => {
// Module: crate::um::ws2spi
// Provides: {"other_58145"}
// Dependencies: {}
extern "system" { # [cfg (target_pointer_width = "64")] pub fn WSCInstallProvider64_32 (lpProviderId : LPGUID , lpszProviderDllPath : * const WCHAR , lpProtocolInfoList : LPWSAPROTOCOL_INFOW , dwNumberOfEntries : DWORD , lpErrno : LPINT ,) -> c_int ; pub fn WSCGetProviderPath (lpProviderId : LPGUID , lpszProviderDllPath : * mut WCHAR , lpProviderDllPathLen : LPINT , lpErrno : LPINT ,) -> c_int ; }
};
}
