// Generated macro for other_58147 (other)
macro_rules! Depcrate_um_ws2spiother_58147 {
() => {
// Module: crate::um::ws2spi
// Provides: {"other_58147"}
// Dependencies: {}
extern "system" { # [cfg (target_pointer_width = "64")] pub fn WSCGetProviderPath32 (lpProviderId : LPGUID , lpszProviderDllPath : * mut WCHAR , lpProviderDllPathLen : LPINT , lpErrno : LPINT ,) -> c_int ; pub fn WSCUpdateProvider (lpProviderId : LPGUID , lpszProviderDllPath : * const WCHAR , lpProtocolInfoList : LPWSAPROTOCOL_INFOW , dwNumberOfEntries : DWORD , lpErrno : LPINT ,) -> c_int ; }
};
}
