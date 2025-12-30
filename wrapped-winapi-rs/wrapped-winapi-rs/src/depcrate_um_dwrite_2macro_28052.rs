// Generated macro for macro_28052 (macro)
macro_rules! Depcrate_um_dwrite_2macro_28052 {
() => {
// Module: crate::um::dwrite_2
// Provides: {"macro_28052"}
// Dependencies: {}
RIDL ! { # [uuid (0xd8b768ff , 0x64bc , 0x4e66 , 0x98 , 0x2b , 0xec , 0x8e , 0x87 , 0xf6 , 0x93 , 0xf7)] interface IDWriteFontFace2 (IDWriteFontFace2Vtbl) : IDWriteFontFace1 (IDWriteFontFace1Vtbl) { fn IsColorFont () -> BOOL , fn GetColorPaletteCount () -> UINT32 , fn GetPaletteEntryCount () -> UINT32 , fn GetPaletteEntries (colorPaletteIndex : UINT32 , firstEntryIndex : UINT32 , entryCount : UINT32 , paletteEntries : * mut DWRITE_COLOR_F ,) -> HRESULT , fn GetRecommendedRenderingMode (fontEmSize : FLOAT , dpiX : FLOAT , dpiY : FLOAT , transform : * const DWRITE_MATRIX , isSideways : BOOL , outlineThreshold : DWRITE_OUTLINE_THRESHOLD , measuringMode : DWRITE_MEASURING_MODE , renderingParams : * mut IDWriteRenderingParams , renderingMode : * mut DWRITE_RENDERING_MODE , gridFitMode : * mut DWRITE_GRID_FIT_MODE ,) -> HRESULT , } }
};
}
