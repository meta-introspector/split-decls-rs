// Generated macro for macro_24038 (macro)
macro_rules! Depcrate_um_d2d1_3macro_24038 {
() => {
// Module: crate::um::d2d1_3
// Provides: {"macro_24038"}
// Dependencies: {}
RIDL ! { # [uuid (0xb499923b , 0x7029 , 0x478f , 0xa8 , 0xb3 , 0x43 , 0x2c , 0x7c , 0x5f , 0x53 , 0x12)] interface ID2D1Ink (ID2D1InkVtbl) : ID2D1Resource (ID2D1ResourceVtbl) { fn SetStartPoint (startPoint : * const D2D1_INK_POINT ,) -> () , fn GetStartPoint () -> D2D1_INK_POINT , fn AddSegments (segments : * const D2D1_INK_BEZIER_SEGMENT , segmentsCount : UINT32 ,) -> HRESULT , fn RemoveSegmentsAtEnd (segmentsCount : UINT32 ,) -> HRESULT , fn SetSegments (startSegment : UINT32 , segments : * const D2D1_INK_BEZIER_SEGMENT , segmentsCount : UINT32 ,) -> HRESULT , fn SetSegmentAtEnd (segment : * const D2D1_INK_BEZIER_SEGMENT ,) -> HRESULT , fn GetSegmentCount () -> UINT32 , fn GetSegments (startSegment : UINT32 , segments : * mut D2D1_INK_BEZIER_SEGMENT , segmentsCount : UINT32 ,) -> HRESULT , fn StreamAsGeometry (inkStyle : * mut ID2D1InkStyle , worldTransform : * const D2D1_MATRIX_3X2_F , flatteningTolerance : FLOAT , geometrySink : * mut ID2D1SimplifiedGeometrySink ,) -> HRESULT , fn GetBounds (inkStyle : * mut ID2D1InkStyle , worldTransform : * const D2D1_MATRIX_3X2_F , bounds : * mut D2D1_RECT_F ,) -> HRESULT , } }
};
}
