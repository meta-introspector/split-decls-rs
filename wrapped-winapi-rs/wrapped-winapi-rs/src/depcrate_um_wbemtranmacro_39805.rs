// Generated macro for macro_39805 (macro)
macro_rules! Depcrate_um_wbemtranmacro_39805 {
() => {
// Module: crate::um::wbemtran
// Provides: {"macro_39805"}
// Dependencies: {}
RIDL ! { # [uuid (0xf309ad18 , 0xd86a , 0x11d0 , 0xa0 , 0x75 , 0x00 , 0xc0 , 0x4f , 0xb6 , 0x88 , 0x20)] interface IWbemLevel1Login (IWbemLevel1LoginVtbl) : IUnknown (IUnknownVtbl) { fn EstablishPosition (wszLocaleList : LPWSTR , dwNumLocales : DWORD , reserved : * mut DWORD ,) -> HRESULT , fn RequestChallenge (wszNetworkResource : LPWSTR , wszUser : LPWSTR , Nonce : WBEM_128BITS ,) -> HRESULT , fn WBEMLogin (wszPreferredLocale : LPWSTR , AccessToken : WBEM_128BITS , lFlags : c_long , pCtx : * mut IWbemContext , ppNamespace : * mut * mut IWbemServices ,) -> HRESULT , fn NTLMLogin (wszNetworkResource : LPWSTR , wszPreferredLocale : LPWSTR , lFlags : c_long , pCtx : * mut IWbemContext , ppNamespace : * mut * mut IWbemServices ,) -> HRESULT , } }
};
}
