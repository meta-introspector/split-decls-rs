// Generated macro for macro_39712 (macro)
macro_rules! Depcrate_um_wbemdispmacro_39712 {
() => {
// Module: crate::um::wbemdisp
// Provides: {"macro_39712"}
// Dependencies: {}
RIDL ! { # [uuid (0x14d8250e , 0xd9c2 , 0x11d3 , 0xb3 , 0x8f , 0x00 , 0x10 , 0x5a , 0x1f , 0x47 , 0x3a)] interface ISWbemRefresher (ISWbemRefresherVtbl) : IDispatch (IDispatchVtbl) { fn get__NewEnum (pUnk : * mut * mut IUnknown ,) -> HRESULT , fn Item (iIndex : c_long , objWbemRefreshableItem : * mut * mut ISWbemRefreshableItem ,) -> HRESULT , fn get_Count (iCount : * mut c_long ,) -> HRESULT , fn Add (objWbemServices : * mut ISWbemServicesEx , bsInstancePath : BSTR , iFlags : c_long , objWbemNamedValueSet : * mut IDispatch , objWbemRefreshableItem : * mut * mut ISWbemRefreshableItem ,) -> HRESULT , fn AddEnum (objWbemServices : * mut ISWbemServicesEx , bsClassName : BSTR , iFlags : c_long , objWbemNamedValueSet : * mut IDispatch , objWbemRefreshableItem : * mut * mut ISWbemRefreshableItem ,) -> HRESULT , fn Remove (iIndex : c_long , iFlags : c_long ,) -> HRESULT , fn Refresh (iFlags : c_long ,) -> HRESULT , fn get_AutoReconnect (bCount : * mut VARIANT_BOOL ,) -> HRESULT , fn put_AutoReconnect (bCount : VARIANT_BOOL ,) -> HRESULT , fn DeleteAll () -> HRESULT , } }
};
}
