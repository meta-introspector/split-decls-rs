// Generated macro for macro_39807 (macro)
macro_rules! Depcrate_um_wbemtranmacro_39807 {
() => {
// Module: crate::um::wbemtran
// Provides: {"macro_39807"}
// Dependencies: {}
RIDL ! { # [uuid (0xd8ec9cb1 , 0xb135 , 0x4f10 , 0x8b , 0x1b , 0xc7 , 0x18 , 0x8b , 0xb0 , 0xd1 , 0x86)] interface IWbemConnectorLogin (IWbemConnectorLoginVtbl) : IUnknown (IUnknownVtbl) { fn ConnectorLogin (wszNetworkResource : LPWSTR , wszPreferredLocale : LPWSTR , lFlags : c_long , pCtx : * mut IWbemContext , riid : REFIID , pInterface : * mut * mut c_void ,) -> HRESULT , } }
};
}
