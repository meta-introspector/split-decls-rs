// Generated macro for macro_24061 (macro)
macro_rules! Depcrate_um_d2d1_3macro_24061 {
() => {
// Module: crate::um::d2d1_3
// Provides: {"macro_24061"}
// Dependencies: {}
RIDL ! { # [uuid (0x7836d248 , 0x68cc , 0x4df6 , 0xb9 , 0xe8 , 0xde , 0x99 , 0x1b , 0xf6 , 0x2e , 0xb7)] interface ID2D1DeviceContext5 (ID2D1DeviceContext5Vtbl) : ID2D1DeviceContext4 (ID2D1DeviceContext4Vtbl) { fn CreateSvgDocument (inputXmlStream : * mut IStream , viewportSize : D2D1_SIZE_F , svgDocument : * mut * mut ID2D1SvgDocument ,) -> HRESULT , fn DrawSvgDocument (svgDocument : * mut ID2D1SvgDocument ,) -> () , fn CreateColorContextFromDxgiColorSpace (colorSpace : DXGI_COLOR_SPACE_TYPE , colorContext : * mut * mut ID2D1ColorContext1 ,) -> HRESULT , fn CreateColorContextFromSimpleColorProfile (simpleProfile : * const D2D1_SIMPLE_COLOR_PROFILE , colorContext : * mut * mut ID2D1ColorContext1 ,) -> HRESULT , } }
};
}
