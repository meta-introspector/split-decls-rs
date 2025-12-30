// Generated macro for other_27182 (other)
macro_rules! Depcrate_um_dcompother_27182 {
() => {
// Module: crate::um::dcomp
// Provides: {"other_27182"}
// Dependencies: {}
extern "system" { pub fn DCompositionCreateDevice (dxgiDevice : * const IDXGIDevice , iid : REFIID , dcompositionDevice : * mut * mut c_void ,) -> HRESULT ; pub fn DCompositionCreateDevice2 (renderingDevice : * const IUnknown , iid : REFIID , dcompositionDevice : * mut * mut c_void ,) -> HRESULT ; pub fn DCompositionCreateDevice3 (renderingDevice : * const IUnknown , iid : REFIID , dcompositionDevice : * mut * mut c_void ,) -> HRESULT ; pub fn DCompositionGetFrameStatistics (statistics : * const DCOMPOSITION_FRAME_STATISTICS , minSafeFeaturelLevel : * const D3D_FEATURE_LEVEL , maxHardwareFeaturelLevel : * const D3D_FEATURE_LEVEL ,) -> HRESULT ; pub fn DCompositionCreateSurfaceHandle (desiredAccess : DWORD , securityAttributes : * const SECURITY_ATTRIBUTES , surfaceHandle : * mut HANDLE ,) -> HRESULT ; pub fn DCompositionAttachMouseWheelToHwnd (visual : * const IDCompositionVisual , hwnd : HWND , enable : BOOL ,) -> HRESULT ; pub fn DCompositionAttachMouseDragToHwnd (visual : * const IDCompositionVisual , hwnd : HWND , enable : BOOL ,) -> HRESULT ; }
};
}
