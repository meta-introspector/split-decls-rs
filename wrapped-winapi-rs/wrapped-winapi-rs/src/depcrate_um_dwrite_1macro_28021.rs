// Generated macro for macro_28021 (macro)
macro_rules! Depcrate_um_dwrite_1macro_28021 {
() => {
// Module: crate::um::dwrite_1
// Provides: {"macro_28021"}
// Dependencies: {}
RIDL ! { # [uuid (0x30572f99 , 0xdac6 , 0x41db , 0xa1 , 0x6e , 0x04 , 0x86 , 0x30 , 0x7e , 0x60 , 0x6a)] interface IDWriteFactory1 (IDWriteFactory1Vtbl) : IDWriteFactory (IDWriteFactoryVtbl) { fn GetEudcFontCollection (fontCollection : * mut * mut IDWriteFontCollection , checkForUpdates : BOOL ,) -> HRESULT , fn CreateCustomRenderingParams (gamma : FLOAT , enhancedContrast : FLOAT , enhancedContrastGrayscale : FLOAT , clearTypeLevel : FLOAT , pixelGeometry : DWRITE_PIXEL_GEOMETRY , renderingMode : DWRITE_RENDERING_MODE , renderingParams : * mut * mut IDWriteRenderingParams1 ,) -> HRESULT , } }
};
}
