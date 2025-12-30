// Generated macro for macro_39672 (macro)
macro_rules! Depcrate_um_wbemdispmacro_39672 {
() => {
// Module: crate::um::wbemdisp
// Provides: {"macro_39672"}
// Dependencies: {}
RIDL ! { # [uuid (0x76a6415f , 0xcb41 , 0x11d1 , 0x8b , 0x02 , 0x00 , 0x60 , 0x08 , 0x06 , 0xd9 , 0xb6)] interface ISWbemObjectSet (ISWbemObjectSetVtbl) : IDispatch (IDispatchVtbl) { fn get__NewEnum (pUnk : * mut * mut IUnknown ,) -> HRESULT , fn Item (strObjectPath : BSTR , iFlags : c_long , objWbemObject : * mut * mut ISWbemObject ,) -> HRESULT , fn get_Count (iCount : * mut c_long ,) -> HRESULT , fn get_Security_ (objWbemSecurity : * mut * mut ISWbemSecurity ,) -> HRESULT , fn ItemIndex (lIndex : c_long , objWbemObject : * mut * mut ISWbemObject ,) -> HRESULT , } }
};
}
