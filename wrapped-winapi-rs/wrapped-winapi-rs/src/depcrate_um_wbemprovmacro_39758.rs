// Generated macro for macro_39758 (macro)
macro_rules! Depcrate_um_wbemprovmacro_39758 {
() => {
// Module: crate::um::wbemprov
// Provides: {"macro_39758"}
// Dependencies: {}
RIDL ! { # [uuid (0xce61e841 , 0x65bc , 0x11d0 , 0xb6 , 0xbd , 0x00 , 0xaa , 0x00 , 0x32 , 0x40 , 0xc7)] interface IWbemPropertyProvider (IWbemPropertyProviderVtbl) : IUnknown (IUnknownVtbl) { fn GetProperty (lFlags : c_long , strLocale : BSTR , strClassMapping : BSTR , strInstMapping : BSTR , strPropMapping : BSTR , pvValue : * mut VARIANT ,) -> HRESULT , fn PutProperty (lFlags : c_long , strLocale : BSTR , strClassMapping : BSTR , strInstMapping : BSTR , strPropMapping : BSTR , pvValue : * const VARIANT ,) -> HRESULT , } }
};
}
