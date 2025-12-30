// Generated macro for macro_27442 (macro)
macro_rules! Depcrate_um_dispexmacro_27442 {
() => {
// Module: crate::um::dispex
// Provides: {"macro_27442"}
// Dependencies: {}
RIDL ! { # [uuid (0xa6ef9861 , 0xc720 , 0x11d0 , 0x93 , 0x37 , 0x00 , 0xa0 , 0xc9 , 0x0d , 0xca , 0xa9)] interface IDispError (IDispErrorVtbl) : IUnknown (IUnknownVtbl) { fn QueryErrorInfo (guidErrorType : GUID , ppde : * mut * mut IDispError ,) -> HRESULT , fn GetNext (ppde : * mut * mut IDispError ,) -> HRESULT , fn GetHresult (phr : * mut HRESULT ,) -> HRESULT , fn GetSource (pbstrSource : * mut BSTR ,) -> HRESULT , fn GetHelpInfo (pbstrFileName : * mut BSTR , pdwContext : * mut DWORD ,) -> HRESULT , fn GetDescription (pbstrDescription : * mut BSTR ,) -> HRESULT , } }
};
}
