// Generated macro for macro_39704 (macro)
macro_rules! Depcrate_um_wbemdispmacro_39704 {
() => {
// Module: crate::um::wbemdisp
// Provides: {"macro_39704"}
// Dependencies: {}
RIDL ! { # [uuid (0x26ee67bf , 0x5804 , 0x11d2 , 0x8b , 0x4a , 0x00 , 0x60 , 0x08 , 0x06 , 0xd9 , 0xb6)] interface ISWbemPrivilegeSet (ISWbemPrivilegeSetVtbl) : IDispatch (IDispatchVtbl) { fn get__NewEnum (pUnk : * mut * mut IUnknown ,) -> HRESULT , fn Item (iPrivilege : WbemPrivilegeEnum , objWbemPrivilege : * mut * mut ISWbemPrivilege ,) -> HRESULT , fn get_Count (iCount : * mut c_long ,) -> HRESULT , fn Add (iPrivilege : WbemPrivilegeEnum , bIsEnabled : VARIANT_BOOL , objWbemPrivilege : * mut * mut ISWbemPrivilege ,) -> HRESULT , fn Remove (iPrivilege : WbemPrivilegeEnum ,) -> HRESULT , fn DeleteAll () -> HRESULT , fn AddAsString (strPrivilege : BSTR , bIsEnabled : VARIANT_BOOL , objWbemPrivilege : * mut * mut ISWbemPrivilege ,) -> HRESULT , } }
};
}
