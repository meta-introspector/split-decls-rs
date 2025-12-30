// Generated macro for macro_39682 (macro)
macro_rules! Depcrate_um_wbemdispmacro_39682 {
() => {
// Module: crate::um::wbemdisp
// Provides: {"macro_39682"}
// Dependencies: {}
RIDL ! { # [uuid (0x1a388f98 , 0xd4ba , 0x11d1 , 0x8b , 0x09 , 0x00 , 0x60 , 0x08 , 0x06 , 0xd9 , 0xb6)] interface ISWbemProperty (ISWbemPropertyVtbl) : IDispatch (IDispatchVtbl) { fn get_Value (varValue : * mut VARIANT ,) -> HRESULT , fn put_Value (varValue : * mut VARIANT ,) -> HRESULT , fn get_Name (strName : * mut BSTR ,) -> HRESULT , fn get_IsLocal (bIsLocal : * mut VARIANT_BOOL ,) -> HRESULT , fn get_Origin (strOrigin : * mut BSTR ,) -> HRESULT , fn get_CIMType (iCimType : * mut WbemCimtypeEnum ,) -> HRESULT , fn get_Qualifiers_ (objWbemQualifierSet : * mut * mut ISWbemQualifierSet ,) -> HRESULT , fn get_IsArray (bIsArray : * mut VARIANT_BOOL ,) -> HRESULT , } }
};
}
