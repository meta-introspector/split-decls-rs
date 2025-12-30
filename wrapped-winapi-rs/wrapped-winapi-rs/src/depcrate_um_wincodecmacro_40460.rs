// Generated macro for macro_40460 (macro)
macro_rules! Depcrate_um_wincodecmacro_40460 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40460"}
// Dependencies: {}
RIDL ! { # [uuid (0x23bc3f0a , 0x698b , 0x4357 , 0x88 , 0x6b , 0xf2 , 0x4d , 0x50 , 0x67 , 0x13 , 0x34)] interface IWICComponentInfo (IWICComponentInfoVtbl) : IUnknown (IUnknownVtbl) { fn GetComponentType (pType : * mut WICComponentType ,) -> HRESULT , fn GetCLSID (pclsid : * mut CLSID ,) -> HRESULT , fn GetSigningStatus (pStatus : * mut DWORD ,) -> HRESULT , fn GetAuthor (cchAuthor : UINT , wzAuthor : * mut WCHAR , pcchActual : * mut UINT ,) -> HRESULT , fn GetVendorGUID (pguidVendor : * mut GUID ,) -> HRESULT , fn GetVersion (cchVersion : UINT , wzVersion : * mut WCHAR , pcchActual : * mut UINT ,) -> HRESULT , fn GetSpecVersion (cchSpecVersion : UINT , wzSpecVersion : * mut WCHAR , pcchActual : * mut UINT ,) -> HRESULT , fn GetFriendlyName (cchFriendlyName : UINT , wzFriendlyName : * mut WCHAR , pcchActual : * mut UINT ,) -> HRESULT , } }
};
}
