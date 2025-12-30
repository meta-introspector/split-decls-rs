// Generated macro for macro_39692 (macro)
macro_rules! Depcrate_um_wbemdispmacro_39692 {
() => {
// Module: crate::um::wbemdisp
// Provides: {"macro_39692"}
// Dependencies: {}
RIDL ! { # [uuid (0x5791bc27 , 0xce9c , 0x11d1 , 0x97 , 0xbf , 0x00 , 0x00 , 0xf8 , 0x1e , 0x84 , 0x9c)] interface ISWbemObjectPath (ISWbemObjectPathVtbl) : IDispatch (IDispatchVtbl) { fn get_Path (strPath : * mut BSTR ,) -> HRESULT , fn put_Path (strPath : BSTR ,) -> HRESULT , fn get_RelPath (strRelPath : * mut BSTR ,) -> HRESULT , fn put_RelPath (strRelPath : BSTR ,) -> HRESULT , fn get_Server (strServer : * mut BSTR ,) -> HRESULT , fn put_Server (strServer : BSTR ,) -> HRESULT , fn get_Namespace (strNamespace : * mut BSTR ,) -> HRESULT , fn put_Namespace (strNamespace : BSTR ,) -> HRESULT , fn get_ParentNamespace (strParentNamespace : * mut BSTR ,) -> HRESULT , fn get_DisplayName (strDisplayName : * mut BSTR ,) -> HRESULT , fn put_DisplayName (strDisplayName : BSTR ,) -> HRESULT , fn get_Class (strClass : * mut BSTR ,) -> HRESULT , fn put_Class (strClass : BSTR ,) -> HRESULT , fn get_IsClass (bIsClass : * mut VARIANT_BOOL ,) -> HRESULT , fn SetAsClass () -> HRESULT , fn get_IsSingleton (bIsSingleton : * mut VARIANT_BOOL ,) -> HRESULT , fn SetAsSingleton () -> HRESULT , fn get_Keys (objWbemNamedValueSet : * mut * mut ISWbemNamedValueSet ,) -> HRESULT , fn get_Security_ (objWbemSecurity : * mut * mut ISWbemSecurity ,) -> HRESULT , fn get_Locale (strLocale : * mut BSTR ,) -> HRESULT , fn put_Locale (strLocale : BSTR ,) -> HRESULT , fn get_Authority (strAuthority : * mut BSTR ,) -> HRESULT , fn put_Authority (strAuthority : BSTR ,) -> HRESULT , } }
};
}
