// Generated macro for other_46433 (other)
macro_rules! Depcrate_um_wingdiother_46433 {
() => {
// Module: crate::um::wingdi
// Provides: {"other_46433"}
// Dependencies: {}
extern "system" { pub fn wglDescribeLayerPlane (hdc : HDC , iPixelFormat : c_int , iLayerPlane : c_int , nBytes : UINT , plpd : LPLAYERPLANEDESCRIPTOR ,) -> BOOL ; pub fn wglSetLayerPaletteEntries (hdc : HDC , iLayerPlane : c_int , iStart : c_int , cEntries : c_int , pcr : * const COLORREF ,) -> c_int ; pub fn wglGetLayerPaletteEntries (hdc : HDC , iLayerPlane : c_int , iStart : c_int , cEntries : c_int , pcr : * const COLORREF ,) -> c_int ; pub fn wglRealizeLayerPalette (hdc : HDC , iLayerPlane : c_int , bRealize : BOOL ,) -> BOOL ; pub fn wglSwapLayerBuffers (hdc : HDC , fuPlanes : UINT ,) -> BOOL ; }
};
}
