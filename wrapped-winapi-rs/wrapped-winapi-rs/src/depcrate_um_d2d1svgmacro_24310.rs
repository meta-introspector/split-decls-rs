// Generated macro for macro_24310 (macro)
macro_rules! Depcrate_um_d2d1svgmacro_24310 {
() => {
// Module: crate::um::d2d1svg
// Provides: {"macro_24310"}
// Dependencies: {}
RIDL ! { # [uuid (0xc095e4f4 , 0xbb98 , 0x43d6 , 0x97 , 0x45 , 0x4d , 0x1b , 0x84 , 0xec , 0x98 , 0x88)] interface ID2D1SvgPathData (ID2D1SvgPathDataVtbl) : ID2D1SvgAttribute (ID2D1SvgAttributeVtbl) { fn RemoveSegmentDataAtEnd (dataCount : UINT32 ,) -> HRESULT , fn UpdateSegmentData (data : * const FLOAT , dataCount : UINT32 , startIndex : UINT32 ,) -> HRESULT , fn GetSegmentData (data : * mut FLOAT , dataCount : UINT32 , startIndex : UINT32 ,) -> HRESULT , fn GetSegmentDataCount () -> UINT32 , fn RemoveCommandsAtEnd (commandsCount : UINT32 ,) -> HRESULT , fn UpdateCommands (commands : * const D2D1_SVG_PATH_COMMAND , commandsCount : UINT32 , startIndex : UINT32 ,) -> HRESULT , fn GetCommands (commands : * mut D2D1_SVG_PATH_COMMAND , commandsCount : UINT32 , startIndex : UINT32 ,) -> HRESULT , fn GetCommandsCount () -> UINT32 , fn CreatePathGeometry (fillMode : D2D1_FILL_MODE , pathGeometry : * mut * mut ID2D1PathGeometry1 ,) -> HRESULT , } }
};
}
