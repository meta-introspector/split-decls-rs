// Generated macro for other_45996 (other)
macro_rules! Depcrate_um_wingdiother_45996 {
() => {
// Module: crate::um::wingdi
// Provides: {"other_45996"}
// Dependencies: {}
extern "system" { pub fn GetKerningPairsA (hdc : HDC , nPairs : DWORD , lpKernPair : LPKERNINGPAIR ,) -> DWORD ; pub fn GetKerningPairsW (hdc : HDC , nPairs : DWORD , lpKernPair : LPKERNINGPAIR ,) -> DWORD ; pub fn GetDCOrgEx (hdc : HDC , lppt : LPPOINT ,) -> BOOL ; pub fn FixBrushOrgEx (hdc : HDC , x : c_int , y : c_int , ptl : LPPOINT ,) -> BOOL ; pub fn UnrealizeObject (h : HGDIOBJ ,) -> BOOL ; pub fn GdiFlush () -> BOOL ; pub fn GdiSetBatchLimit (dw : DWORD ,) -> DWORD ; pub fn GdiGetBatchLimit () -> DWORD ; }
};
}
