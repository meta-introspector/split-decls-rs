// Generated macro for macro_39676 (macro)
macro_rules! Depcrate_um_wbemdispmacro_39676 {
() => {
// Module: crate::um::wbemdisp
// Provides: {"macro_39676"}
// Dependencies: {}
RIDL ! { # [uuid (0xcf2376ea , 0xce8c , 0x11d1 , 0x8b , 0x05 , 0x00 , 0x60 , 0x08 , 0x06 , 0xd9 , 0xb6)] interface ISWbemNamedValueSet (ISWbemNamedValueSetVtbl) : IDispatch (IDispatchVtbl) { fn get__NewEnum (pUnk : * mut * mut IUnknown ,) -> HRESULT , fn Item (strName : BSTR , iFlags : c_long , objWbemNamedValue : * mut * mut ISWbemNamedValue ,) -> HRESULT , fn get_Count (iCount : * mut c_long ,) -> HRESULT , fn Add (strName : BSTR , varValue : * mut VARIANT , iFlags : c_long , objWbemNamedValue : * mut * mut ISWbemNamedValue ,) -> HRESULT , fn Remove (strName : BSTR , iFlags : c_long ,) -> HRESULT , fn Clone (objWbemNamedValueSet : * mut * mut ISWbemNamedValueSet ,) -> HRESULT , fn DeleteAll () -> HRESULT , } }
};
}
