// Generated macro for macro_27204 (macro)
macro_rules! Depcrate_um_dcompmacro_27204 {
() => {
// Module: crate::um::dcomp
// Provides: {"macro_27204"}
// Dependencies: {}
RIDL ! { # [uuid (0x5f4633fe , 0x1e08 , 0x4cb8 , 0x8c , 0x75 , 0xce , 0x24 , 0x33 , 0x3f , 0x56 , 0x02)] interface IDCompositionDesktopDevice (IDCompositionDesktopDeviceVtbl) : IDCompositionDevice2 (IDCompositionDevice2Vtbl) { fn CreateTargetForHwnd (hwnd : HWND , topmost : BOOL , target : * mut * mut IDCompositionTarget ,) -> HRESULT , fn CreateSurfaceFromHandle (handle : HANDLE , surface : * mut * mut IUnknown ,) -> HRESULT , fn CreateSurfaceFromHwnd (hwnd : HWND , surface : * mut * mut IUnknown ,) -> HRESULT , } }
};
}
