// Generated macro for macro_39684 (macro)
macro_rules! Depcrate_um_wbemdispmacro_39684 {
() => {
// Module: crate::um::wbemdisp
// Provides: {"macro_39684"}
// Dependencies: {}
RIDL ! { # [uuid (0xdea0a7b2 , 0xd4ba , 0x11d1 , 0x8b , 0x09 , 0x00 , 0x60 , 0x08 , 0x06 , 0xd9 , 0xb6)] interface ISWbemPropertySet (ISWbemPropertySetVtbl) : IDispatch (IDispatchVtbl) { fn get__NewEnum (pUnk : * mut * mut IUnknown ,) -> HRESULT , fn Item (strName : BSTR , iFlags : c_long , objWbemProperty : * mut * mut ISWbemProperty ,) -> HRESULT , fn get_Count (iCount : * mut c_long ,) -> HRESULT , fn Add (strName : BSTR , iCIMType : WbemCimtypeEnum , bIsArray : VARIANT_BOOL , iFlags : c_long , objWbemProperty : * mut * mut ISWbemProperty ,) -> HRESULT , fn Remove (strName : BSTR , iFlags : c_long ,) -> HRESULT , } }
};
}
