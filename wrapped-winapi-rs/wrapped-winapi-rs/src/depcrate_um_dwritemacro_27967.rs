// Generated macro for macro_27967 (macro)
macro_rules! Depcrate_um_dwritemacro_27967 {
() => {
// Module: crate::um::dwrite
// Provides: {"macro_27967"}
// Dependencies: {}
RIDL ! { # [uuid (0x7d97dbf7 , 0xe085 , 0x42d4 , 0x81 , 0xe3 , 0x6a , 0x88 , 0x3b , 0xde , 0xd1 , 0x18)] interface IDWriteGlyphRunAnalysis (IDWriteGlyphRunAnalysisVtbl) : IUnknown (IUnknownVtbl) { fn GetAlphaTextureBounds (textureType : DWRITE_TEXTURE_TYPE , textureBounds : * mut RECT ,) -> HRESULT , fn CreateAlphaTexture (textureType : DWRITE_TEXTURE_TYPE , textureBounds : * const RECT , alphaValues : * mut BYTE , bufferSize : UINT32 ,) -> HRESULT , fn GetAlphaBlendParams (renderingParams : * mut IDWriteRenderingParams , blendGamma : * mut FLOAT , blendEnhancedContrast : * mut FLOAT , blendClearTypeLevel : * mut FLOAT ,) -> HRESULT , } }
};
}
