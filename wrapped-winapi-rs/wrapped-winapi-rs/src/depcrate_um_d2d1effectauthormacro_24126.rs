// Generated macro for macro_24126 (macro)
macro_rules! Depcrate_um_d2d1effectauthormacro_24126 {
() => {
// Module: crate::um::d2d1effectauthor
// Provides: {"macro_24126"}
// Dependencies: {}
RIDL ! { # [uuid (0x693ce632 , 0x7f2f , 0x45de , 0x93 , 0xfe , 0x18 , 0xd8 , 0x8b , 0x37 , 0xaa , 0x21)] interface ID2D1DrawInfo (ID2D1DrawInfoVtbl) : ID2D1RenderInfo (ID2D1RenderInfoVtbl) { fn SetPixelShaderConstantBuffer (buffer : * const BYTE , bufferCount : UINT32 ,) -> HRESULT , fn SetResourceTexture (textureIndex : UINT32 , resourceTexture : * mut ID2D1ResourceTexture ,) -> HRESULT , fn SetVertexShaderConstantBuffer (buffer : * const BYTE , bufferCount : UINT32 ,) -> HRESULT , fn SetPixelShader (shaderId : REFGUID , pixelOptions : D2D1_PIXEL_OPTIONS ,) -> HRESULT , fn SetVertexProcessing (vertexBuffer : * mut ID2D1VertexBuffer , vertexOptions : D2D1_VERTEX_OPTIONS , blendDescription : * const D2D1_BLEND_DESCRIPTION , vertexRange : * const D2D1_VERTEX_RANGE , vertexShader : * const GUID ,) -> HRESULT , } }
};
}
