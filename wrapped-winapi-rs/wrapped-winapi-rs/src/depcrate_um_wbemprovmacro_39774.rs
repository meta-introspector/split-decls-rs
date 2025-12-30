// Generated macro for macro_39774 (macro)
macro_rules! Depcrate_um_wbemprovmacro_39774 {
() => {
// Module: crate::um::wbemprov
// Provides: {"macro_39774"}
// Dependencies: {}
RIDL ! { # [uuid (0x49353c93 , 0x516b , 0x11d1 , 0xae , 0xa6 , 0x00 , 0xc0 , 0x4f , 0xb6 , 0x88 , 0x20)] interface IWbemHiPerfProvider (IWbemHiPerfProviderVtbl) : IUnknown (IUnknownVtbl) { fn QueryInstances (pNamespace : * mut IWbemServices , wszClass : * mut WCHAR , lFlags : c_long , pCtx : * mut IWbemContext , pSink : * mut IWbemObjectSink ,) -> HRESULT , fn CreateRefresher (pNamespace : * mut IWbemServices , lFlags : c_long , ppRefresher : * mut * mut IWbemRefresher ,) -> HRESULT , fn CreateRefreshableObject (pNamespace : * mut IWbemServices , pTemplate : * mut IWbemObjectAccess , pRefresher : * mut IWbemRefresher , lFlags : c_long , pContext : * mut IWbemContext , ppRefreshable : * mut * mut IWbemObjectAccess , plId : * mut c_long ,) -> HRESULT , fn StopRefreshing (pRefresher : * mut IWbemRefresher , lId : c_long , lFlags : c_long ,) -> HRESULT , fn CreateRefreshableEnum (pNamespace : * mut IWbemServices , wszClass : LPCWSTR , pRefresher : * mut IWbemRefresher , lFlags : c_long , pContext : * mut IWbemContext , pHiPerfEnum : * mut IWbemHiPerfEnum , plId : * mut c_long ,) -> HRESULT , fn GetObjects (pNamespace : * mut IWbemServices , lNumObjects : c_long , apObj : * mut * mut IWbemObjectAccess , lFlags : c_long , pContext : * mut IWbemContext ,) -> HRESULT , } }
};
}
