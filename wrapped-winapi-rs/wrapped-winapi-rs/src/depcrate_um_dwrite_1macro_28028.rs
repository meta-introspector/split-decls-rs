// Generated macro for macro_28028 (macro)
macro_rules! Depcrate_um_dwrite_1macro_28028 {
() => {
// Module: crate::um::dwrite_1
// Provides: {"macro_28028"}
// Dependencies: {}
RIDL ! { # [uuid (0x9064d822 , 0x80a7 , 0x465c , 0xa9 , 0x86 , 0xdf , 0x65 , 0xf7 , 0x8b , 0x8f , 0xeb)] interface IDWriteTextLayout1 (IDWriteTextLayout1Vtbl) : IDWriteTextLayout (IDWriteTextLayoutVtbl) { fn SetPairKerning (isPairKerningEnabled : BOOL , textRange : DWRITE_TEXT_RANGE ,) -> HRESULT , fn GetPairKerning (currentPosition : UINT32 , isPairKerningEnabled : * mut BOOL , textRange : * mut DWRITE_TEXT_RANGE ,) -> HRESULT , fn SetCharacterSpacing (leadingSpacing : FLOAT , trailingSpacing : FLOAT , minimumAdvanceWidth : FLOAT , textRange : DWRITE_TEXT_RANGE ,) -> HRESULT , fn GetCharacterSpacing (currentPosition : UINT32 , leadingSpacing : * mut FLOAT , trailingSpacing : * mut FLOAT , minimumAdvanceWidth : * mut FLOAT , textRange : * mut DWRITE_TEXT_RANGE ,) -> HRESULT , } }
};
}
