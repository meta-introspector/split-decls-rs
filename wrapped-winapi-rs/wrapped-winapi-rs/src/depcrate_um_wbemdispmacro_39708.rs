// Generated macro for macro_39708 (macro)
macro_rules! Depcrate_um_wbemdispmacro_39708 {
() => {
// Module: crate::um::wbemdisp
// Provides: {"macro_39708"}
// Dependencies: {}
RIDL ! { # [uuid (0x269ad56a , 0x8a67 , 0x4129 , 0xbc , 0x8c , 0x05 , 0x06 , 0xdc , 0xfe , 0x98 , 0x80)] interface ISWbemObjectEx (ISWbemObjectExVtbl) : ISWbemObject (ISWbemObjectVtbl) { fn Refresh_ (iFlags : c_long , objWbemNamedValueSet : * mut IDispatch ,) -> HRESULT , fn get_SystemProperties_ (objWbemPropertySet : * mut * mut ISWbemPropertySet ,) -> HRESULT , fn GetText_ (iObjectTextFormat : WbemObjectTextFormatEnum , iFlags : c_long , objWbemNamedValueSet : * mut IDispatch , bsText : * mut BSTR ,) -> HRESULT , fn SetFromText_ (bsText : BSTR , iObjectTextFormat : WbemObjectTextFormatEnum , iFlags : c_long , objWbemNamedValueSet : * mut IDispatch ,) -> HRESULT , } }
};
}
