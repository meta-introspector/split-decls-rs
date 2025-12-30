// Generated macro for other_55838 (other)
macro_rules! Depcrate_um_winuserother_55838 {
() => {
// Module: crate::um::winuser
// Provides: {"other_55838"}
// Dependencies: {}
extern "system" { pub fn EvaluateProximityToRect (controlBoundingBox : * const RECT , pHitTestingInput : * const TOUCH_HIT_TESTING_INPUT , pProximityEval : * mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION ,) -> BOOL ; pub fn EvaluateProximityToPolygon (numVertices : UINT32 , controlPolygon : * const POINT , pHitTestingInput : * const TOUCH_HIT_TESTING_INPUT , pProximityEval : * mut TOUCH_HIT_TESTING_PROXIMITY_EVALUATION ,) -> BOOL ; pub fn PackTouchHitTestingProximityEvaluation (pHitTestingInput : * const TOUCH_HIT_TESTING_INPUT , pProximityEval : * const TOUCH_HIT_TESTING_PROXIMITY_EVALUATION ,) -> LRESULT ; }
};
}
