// Generated macro for macro_39702 (macro)
macro_rules! Depcrate_um_wbemdispmacro_39702 {
() => {
// Module: crate::um::wbemdisp
// Provides: {"macro_39702"}
// Dependencies: {}
RIDL ! { # [uuid (0x26ee67bd , 0x5804 , 0x11d2 , 0x8b , 0x4a , 0x00 , 0x60 , 0x08 , 0x06 , 0xd9 , 0xb6)] interface ISWbemPrivilege (ISWbemPrivilegeVtbl) : IDispatch (IDispatchVtbl) { fn get_IsEnabled (bIsEnabled : * mut VARIANT_BOOL ,) -> HRESULT , fn put_IsEnabled (bIsEnabled : VARIANT_BOOL ,) -> HRESULT , fn get_Name (strDisplayName : * mut BSTR ,) -> HRESULT , fn get_DisplayName (strDisplayName : * mut BSTR ,) -> HRESULT , fn get_Identifier (iPrivilege : * mut WbemPrivilegeEnum ,) -> HRESULT , } }
};
}
