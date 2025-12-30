// Generated macro for macro_36402 (macro)
macro_rules! Depcrate_um_sapi51macro_36402 {
() => {
// Module: crate::um::sapi51
// Provides: {"macro_36402"}
// Dependencies: {}
RIDL ! { # [uuid (0x14056581 , 0xe16c , 0x11d2 , 0xbb , 0x90 , 0x00 , 0xc0 , 0x4f , 0x8e , 0xe6 , 0xc0)] interface ISpDataKey (ISpDataKeyVtbl) : IUnknown (IUnknownVtbl) { fn SetData (pszValueName : LPCWSTR , cbData : ULONG , pData : * const BYTE ,) -> HRESULT , fn GetData (pszValueName : LPCWSTR , pcbData : * mut ULONG , pData : * mut BYTE ,) -> HRESULT , fn SetStringValue (pszValueName : LPCWSTR , pszValue : LPCWSTR ,) -> HRESULT , fn GetStringValue (pszValueName : LPCWSTR , ppszValue : * mut LPWSTR ,) -> HRESULT , fn SetDWORD (pszValueName : LPCWSTR , dwValue : DWORD ,) -> HRESULT , fn GetDWORD (pszValueName : LPCWSTR , pdwValue : * mut DWORD ,) -> HRESULT , fn OpenKey (pszSubKeyName : LPCWSTR , ppSubKey : * mut * mut ISpDataKey ,) -> HRESULT , fn CreateKey (pszSubKey : LPCWSTR , ppSubKey : * mut * mut ISpDataKey ,) -> HRESULT , fn DeleteKey (pszSubKey : LPCWSTR ,) -> HRESULT , fn DeleteValue (pszValueName : LPCWSTR ,) -> HRESULT , fn EnumKeys (Index : ULONG , ppszSubKeyName : * mut LPWSTR ,) -> HRESULT , fn EnumValues (Index : ULONG , ppszValueName : * mut LPWSTR ,) -> HRESULT , } }
};
}
