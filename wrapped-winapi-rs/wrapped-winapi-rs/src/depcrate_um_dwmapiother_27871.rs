// Generated macro for other_27871 (other)
macro_rules! Depcrate_um_dwmapiother_27871 {
() => {
// Module: crate::um::dwmapi
// Provides: {"other_27871"}
// Dependencies: {}
extern "system" { pub fn DwmRenderGesture (gt : GESTURE_TYPE , cContacts : UINT , pdwPointerID : * const DWORD , pPoints : * const POINT ,) -> HRESULT ; pub fn DwmTetherContact (dwPointerID : DWORD , fEnable : BOOL , ptTether : POINT ,) -> HRESULT ; }
};
}
