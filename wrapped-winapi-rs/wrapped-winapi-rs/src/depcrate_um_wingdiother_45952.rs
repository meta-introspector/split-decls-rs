// Generated macro for other_45952 (other)
macro_rules! Depcrate_um_wingdiother_45952 {
() => {
// Module: crate::um::wingdi
// Provides: {"other_45952"}
// Dependencies: {}
extern "system" { pub fn AngleArc (hdc : HDC , X : c_int , Y : c_int , dwRadius : DWORD , eStartAngle : FLOAT , eSweepAngle : FLOAT ,) -> BOOL ; pub fn PolyPolyline (hdc : HDC , lppt : * const POINT , lpdwPolyPoints : * const DWORD , cCount : DWORD ,) -> BOOL ; pub fn GetWorldTransform (hdc : HDC , lpxf : LPXFORM ,) -> BOOL ; pub fn SetWorldTransform (hdc : HDC , lpxf : * const XFORM ,) -> BOOL ; pub fn ModifyWorldTransform (hdc : HDC , lpxf : * const XFORM , mode : DWORD ,) -> BOOL ; pub fn CombineTransform (lpxformResult : LPXFORM , lpxform1 : * const XFORM , lpxform2 : * const XFORM ,) -> BOOL ; }
};
}
