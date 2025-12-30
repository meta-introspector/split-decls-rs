// Generated macro for macro_39772 (macro)
macro_rules! Depcrate_um_wbemprovmacro_39772 {
() => {
// Module: crate::um::wbemprov
// Provides: {"macro_39772"}
// Dependencies: {}
RIDL ! { # [uuid (0x1be41572 , 0x91dd , 0x11d1 , 0xae , 0xb2 , 0x00 , 0xc0 , 0x4f , 0xb6 , 0x88 , 0x20)] interface IWbemProviderInit (IWbemProviderInitVtbl) : IUnknown (IUnknownVtbl) { fn Initialize (wszUser : LPWSTR , lFlags : LONG , wszNamespace : LPWSTR , wszLocale : LPWSTR , pNamespace : * mut IWbemServices , pCtx : * mut IWbemContext , pInitSink : * mut IWbemProviderInitSink ,) -> HRESULT , } }
};
}
