// Generated macro for other_45944 (other)
macro_rules! Depcrate_um_wingdiother_45944 {
() => {
// Module: crate::um::wingdi
// Provides: {"other_45944"}
// Dependencies: {}
extern "system" { pub fn GradientFill (hdc : HDC , pVertex : PTRIVERTEX , nVertex : ULONG , pMesh : PVOID , nMesh : ULONG , ulMode : ULONG ,) -> BOOL ; pub fn GdiAlphaBlend (hdcDest : HDC , xoriginDest : c_int , yoriginDest : c_int , wDest : c_int , hDest : c_int , hdcSrc : HDC , xoriginSrc : c_int , yoriginSrc : c_int , wSrc : c_int , hSrc : c_int , ftn : BLENDFUNCTION ,) -> BOOL ; pub fn GdiTransparentBlt (hdcDest : HDC , xoriginDest : c_int , yoriginDest : c_int , wDest : c_int , hDest : c_int , hdcSrc : HDC , xoriginSrc : c_int , yoriginSrc : c_int , wSrc : c_int , hSrc : c_int , crTransparent : UINT ,) -> BOOL ; pub fn GdiGradientFill (hdc : HDC , pVertex : PTRIVERTEX , nVertex : ULONG , pMesh : PVOID , nCount : ULONG , ulMode : ULONG ,) -> BOOL ; pub fn PlayMetaFileRecord (hdc : HDC , lpHandleTable : LPHANDLETABLE , lpMR : LPMETARECORD , noObjs : UINT ,) -> BOOL ; }
};
}
