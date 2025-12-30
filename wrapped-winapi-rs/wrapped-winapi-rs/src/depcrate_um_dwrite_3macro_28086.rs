// Generated macro for macro_28086 (macro)
macro_rules! Depcrate_um_dwrite_3macro_28086 {
() => {
// Module: crate::um::dwrite_3
// Provides: {"macro_28086"}
// Dependencies: {}
RIDL ! { # [uuid (0xcfee3140 , 0x1157 , 0x47ca , 0x8b , 0x85 , 0x31 , 0xbf , 0xcf , 0x3f , 0x2d , 0x0e)] interface IDWriteStringList (IDWriteStringListVtbl) : IUnknown (IUnknownVtbl) { fn GetCount () -> UINT32 , fn GetLocaleNameLength (listIndex : UINT32 , length : * mut UINT32 ,) -> HRESULT , fn GetLocaleName (listIndex : UINT32 , localeName : * mut WCHAR , size : UINT32 ,) -> HRESULT , fn GetStringLength (listIndex : UINT32 , length : * mut UINT32 ,) -> HRESULT , fn GetString (listIndex : UINT32 , stringBuffer : * mut WCHAR , stringBufferSize : UINT32 ,) -> HRESULT , } }
};
}
