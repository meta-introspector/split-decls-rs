// Generated macro for macro_39592 (macro)
macro_rules! Depcrate_um_wbemclimacro_39592 {
() => {
// Module: crate::um::wbemcli
// Provides: {"macro_39592"}
// Dependencies: {}
RIDL ! { # [uuid (0xdc12a680 , 0x737f , 0x11cf , 0x88 , 0x4d , 0x00 , 0xaa , 0x00 , 0x4b , 0x2e , 0x24)] interface IWbemQualifierSet (IWbemQualifierSetVtbl) : IUnknown (IUnknownVtbl) { fn Get (wszName : LPCWSTR , lFlags : c_long , pVal : * mut VARIANT , plFlavor : * mut c_long ,) -> HRESULT , fn Put (wszName : LPCWSTR , pVal : * mut VARIANT , lFlavor : c_long ,) -> HRESULT , fn Delete (wszName : LPCWSTR ,) -> HRESULT , fn GetNames (lFlags : c_long , pNames : * mut * mut SAFEARRAY ,) -> HRESULT , fn BeginEnumeration (lFlags : c_long ,) -> HRESULT , fn Next (lFlags : c_long , pstrName : * mut BSTR , pVal : * mut VARIANT , plFlavor : * mut c_long ,) -> HRESULT , fn EndEnumeration () -> HRESULT , } }
};
}
