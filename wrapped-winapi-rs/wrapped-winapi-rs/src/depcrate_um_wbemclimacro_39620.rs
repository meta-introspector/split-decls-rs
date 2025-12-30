// Generated macro for macro_39620 (macro)
macro_rules! Depcrate_um_wbemclimacro_39620 {
() => {
// Module: crate::um::wbemcli
// Provides: {"macro_39620"}
// Dependencies: {}
RIDL ! { # [uuid (0x49353c92 , 0x516b , 0x11d1 , 0xae , 0xa6 , 0x00 , 0xc0 , 0x4f , 0xb6 , 0x88 , 0x20)] interface IWbemConfigureRefresher (IWbemConfigureRefresherVtbl) : IUnknown (IUnknownVtbl) { fn AddObjectByPath (pNamespace : * mut IWbemServices , wszPath : LPCWSTR , lFlags : c_long , pContext : * mut IWbemContext , ppRefreshable : * mut * mut IWbemClassObject , plId : * mut c_long ,) -> HRESULT , fn AddObjectByTemplate (pNamespace : * mut IWbemServices , pTemplate : * mut IWbemClassObject , lFlags : c_long , pContext : * mut IWbemContext , ppRefreshable : * mut * mut IWbemClassObject , plId : * mut c_long ,) -> HRESULT , fn AddRefresher (pRefresher : * mut IWbemRefresher , lFlags : c_long , plId : * mut c_long ,) -> HRESULT , fn Remove (lId : c_long , lFlags : c_long ,) -> HRESULT , fn AddEnum (pNamespace : * mut IWbemServices , wszClassName : LPCWSTR , lFlags : c_long , pContext : * mut IWbemContext , ppEnum : * mut * mut IWbemHiPerfEnum , plId : * mut c_long ,) -> HRESULT , } }
};
}
