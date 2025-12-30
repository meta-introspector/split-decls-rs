// Generated macro for other_46378 (other)
macro_rules! Depcrate_um_wingdiother_46378 {
() => {
// Module: crate::um::wingdi
// Provides: {"other_46378"}
// Dependencies: {}
extern "system" { pub fn wglCopyContext (hglrcSrc : HGLRC , hglrcDst : HGLRC , mask : UINT ,) -> BOOL ; pub fn wglCreateContext (hdc : HDC ,) -> HGLRC ; pub fn wglCreateLayerContext (hdc : HDC , iLayerPlane : c_int ,) -> HGLRC ; pub fn wglDeleteContext (hglrc : HGLRC ,) -> BOOL ; pub fn wglGetCurrentContext () -> HGLRC ; pub fn wglGetCurrentDC () -> HDC ; pub fn wglGetProcAddress (lpszProc : LPCSTR ,) -> PROC ; pub fn wglMakeCurrent (hdc : HDC , hglrc : HGLRC ,) -> BOOL ; pub fn wglShareLists (hglrc1 : HGLRC , hglrc2 : HGLRC ,) -> BOOL ; pub fn wglUseFontBitmapsA (hdc : HDC , first : DWORD , count : DWORD , listBase : DWORD ,) -> BOOL ; pub fn wglUseFontBitmapsW (hdc : HDC , first : DWORD , count : DWORD , listBase : DWORD ,) -> BOOL ; pub fn SwapBuffers (hdc : HDC ,) -> BOOL ; }
};
}
