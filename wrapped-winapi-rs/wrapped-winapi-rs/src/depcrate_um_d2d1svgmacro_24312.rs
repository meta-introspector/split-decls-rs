// Generated macro for macro_24312 (macro)
macro_rules! Depcrate_um_d2d1svgmacro_24312 {
() => {
// Module: crate::um::d2d1svg
// Provides: {"macro_24312"}
// Dependencies: {}
RIDL ! { # [uuid (0x86b88e4d , 0xafa4 , 0x4d7b , 0x88 , 0xe4 , 0x68 , 0xa5 , 0x1c , 0x4a , 0x0a , 0xec)] interface ID2D1SvgDocument (ID2D1SvgDocumentVtbl) : ID2D1Resource (ID2D1ResourceVtbl) { fn SetViewportSize (viewportSize : D2D1_SIZE_F ,) -> HRESULT , fn GetViewportSize () -> D2D1_SIZE_F , fn SetRoot (root : * mut ID2D1SvgElement ,) -> HRESULT , fn GetRoot (root : * mut * mut ID2D1SvgElement ,) -> () , fn FindElementById (id : PCWSTR , svgElement : * mut * mut ID2D1SvgElement ,) -> HRESULT , fn Serialize (outputXmlStream : * mut IStream , subtree : * mut ID2D1SvgElement ,) -> HRESULT , fn Deserialize (inputXmlStream : * mut IStream , subtree : * mut * mut ID2D1SvgElement ,) -> HRESULT , fn CreatePaint (paintType : D2D1_SVG_PAINT_TYPE , color : * const D2D1_COLOR_F , id : PCWSTR , paint : * mut * mut ID2D1SvgPaint ,) -> HRESULT , fn CreateStrokeDashArray (dashes : * const D2D1_SVG_LENGTH , dashesCount : UINT32 , strokeDashArray : * mut * mut ID2D1SvgStrokeDashArray ,) -> HRESULT , fn CreatePointCollection (points : * const D2D1_POINT_2F , pountsCount : UINT32 , pointCollection : * mut ID2D1SvgPointCollection ,) -> HRESULT , fn CreatePathData (segmentData : * const FLOAT , segmentDataCount : UINT32 , commands : * const D2D1_SVG_PATH_COMMAND , commandsCount : UINT32 , pathData : * mut * mut ID2D1SvgPathData ,) -> HRESULT , } }
};
}
