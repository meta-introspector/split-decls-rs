// Generated macro for macro_39680 (macro)
macro_rules! Depcrate_um_wbemdispmacro_39680 {
() => {
// Module: crate::um::wbemdisp
// Provides: {"macro_39680"}
// Dependencies: {}
RIDL ! { # [uuid (0x9b16ed16 , 0xd3df , 0x11d1 , 0x8b , 0x08 , 0x00 , 0x60 , 0x08 , 0x06 , 0xd9 , 0xb6)] interface ISWbemQualifierSet (ISWbemQualifierSetVtbl) : IDispatch (IDispatchVtbl) { fn get__NewEnum (pUnk : * mut * mut IUnknown ,) -> HRESULT , fn Item (name : BSTR , iFlags : c_long , objWbemQualifier : * mut * mut ISWbemQualifier ,) -> HRESULT , fn get_Count (iCount : * mut c_long ,) -> HRESULT , fn Add (strName : BSTR , varVal : * mut VARIANT , bPropagatesToSubclass : VARIANT_BOOL , bPropagatesToInstance : VARIANT_BOOL , bIsOverridable : VARIANT_BOOL , iFlags : c_long , objWbemQualifier : * mut * mut ISWbemQualifier ,) -> HRESULT , fn Remove (strName : BSTR , iFlags : c_long ,) -> HRESULT , } }
};
}
