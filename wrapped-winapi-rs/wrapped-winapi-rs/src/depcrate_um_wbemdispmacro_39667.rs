// Generated macro for macro_39667 (macro)
macro_rules! Depcrate_um_wbemdispmacro_39667 {
() => {
// Module: crate::um::wbemdisp
// Provides: {"macro_39667"}
// Dependencies: {}
RIDL ! { # [uuid (0x76a6415b , 0xcb41 , 0x11d1 , 0x8b , 0x02 , 0x00 , 0x60 , 0x08 , 0x06 , 0xd9 , 0xb6)] interface ISWbemLocator (ISWbemLocatorVtbl) : IDispatch (IDispatchVtbl) { fn ConnectServer (strServer : BSTR , strNamespace : BSTR , strUser : BSTR , strPassword : BSTR , strLocale : BSTR , strAuthority : BSTR , iSecurityFlags : c_long , objWbemNamedValueSet : * mut IDispatch , objWbemServices : * mut * mut ISWbemServices ,) -> HRESULT , fn get_Security_ (objWbemSecurity : * mut * mut ISWbemSecurity ,) -> HRESULT , } }
};
}
