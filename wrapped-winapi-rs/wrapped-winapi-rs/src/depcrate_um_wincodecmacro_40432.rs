// Generated macro for macro_40432 (macro)
macro_rules! Depcrate_um_wincodecmacro_40432 {
() => {
// Module: crate::um::wincodec
// Provides: {"macro_40432"}
// Dependencies: {}
RIDL ! { # [uuid (0x00000040 , 0xa8f2 , 0x4877 , 0xba , 0x0a , 0xfd , 0x2b , 0x66 , 0x45 , 0xfb , 0x94)] interface IWICPalette (IWICPaletteVtbl) : IUnknown (IUnknownVtbl) { fn InitializePredefined (ePaletteType : WICBitmapPaletteType , fAddTransparentColor : BOOL ,) -> HRESULT , fn InitializeCustom (pColors : * const WICColor , cCount : UINT ,) -> HRESULT , fn InitializeFromBitmap (pISurface : * const IWICBitmapSource , cCount : UINT , fAddTransparentColor : BOOL ,) -> HRESULT , fn InitializeFromPalette (pIPalette : * const IWICPalette ,) -> HRESULT , fn GetType (pePaletteType : * mut WICBitmapPaletteType ,) -> HRESULT , fn GetColorCount (pcCount : * mut UINT ,) -> HRESULT , fn GetColors (cCount : UINT , pColors : * mut WICColor , pcActualColors : * mut UINT ,) -> HRESULT , fn IsBlackWhite (pfIsBlackWhite : * mut BOOL ,) -> HRESULT , fn IsGrayscale (pfIsGrayscale : * mut BOOL ,) -> HRESULT , fn HasAlpha (pfHasAlpha : * mut BOOL ,) -> HRESULT , } }
};
}
