// Generated macro for macro_39604 (macro)
macro_rules! Depcrate_um_wbemclimacro_39604 {
() => {
// Module: crate::um::wbemcli
// Provides: {"macro_39604"}
// Dependencies: {}
RIDL ! { # [uuid (0x44aca674 , 0xe8fc , 0x11d0 , 0xa0 , 0x7c , 0x00 , 0xc0 , 0x4f , 0xb6 , 0x88 , 0x20)] interface IWbemContext (IWbemContextVtbl) : IUnknown (IUnknownVtbl) { fn Clone (ppNewCopy : * mut * mut IWbemContext ,) -> HRESULT , fn GetNames (lFlags : c_long , pNames : * mut * mut SAFEARRAY ,) -> HRESULT , fn BeginEnumeration (lFlags : c_long ,) -> HRESULT , fn Next (lFlags : c_long , pstrName : * mut BSTR , pValue : * mut VARIANT ,) -> HRESULT , fn EndEnumeration () -> HRESULT , fn SetValue (wszName : LPCWSTR , lFlags : c_long , pValue : * mut VARIANT ,) -> HRESULT , fn GetValue (wszName : LPCWSTR , lFlags : c_long , pValue : * mut VARIANT ,) -> HRESULT , fn DeleteValue (wszName : LPCWSTR , lFlags : c_long ,) -> HRESULT , fn DeleteAll () -> HRESULT , } }
};
}
