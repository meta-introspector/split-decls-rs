// Generated macro for macro_27915 (macro)
macro_rules! Depcrate_um_dwritemacro_27915 {
() => {
// Module: crate::um::dwrite
// Provides: {"macro_27915"}
// Dependencies: {}
RIDL ! { # [uuid (0x08256209 , 0x099a , 0x4b34 , 0xb8 , 0x6d , 0xc2 , 0x2b , 0x11 , 0x0e , 0x77 , 0x71)] interface IDWriteLocalizedStrings (IDWriteLocalizedStringsVtbl) : IUnknown (IUnknownVtbl) { fn GetCount () -> UINT32 , fn FindLocaleName (localeName : * const WCHAR , index : * mut UINT32 , exists : * mut BOOL ,) -> HRESULT , fn GetLocaleNameLength (index : UINT32 , length : * mut UINT32 ,) -> HRESULT , fn GetLocaleName (index : UINT32 , localeName : * mut WCHAR , size : UINT32 ,) -> HRESULT , fn GetStringLength (index : UINT32 , length : * mut UINT32 ,) -> HRESULT , fn GetString (index : UINT32 , stringBuffer : * mut WCHAR , size : UINT32 ,) -> HRESULT , } }
};
}
