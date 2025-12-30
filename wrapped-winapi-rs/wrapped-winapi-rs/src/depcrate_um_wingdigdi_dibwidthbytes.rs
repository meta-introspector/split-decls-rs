// Generated macro for GDI_DIBWIDTHBYTES (function)
macro_rules! Depcrate_um_wingdiGDI_DIBWIDTHBYTES {
() => {
// Module: crate::um::wingdi
// Provides: {"GDI_DIBWIDTHBYTES"}
// Dependencies: {}
# [inline] pub fn GDI_DIBWIDTHBYTES (bi : & BITMAPINFOHEADER) -> DWORD { GDI_WIDTHBYTES ((bi . biWidth as DWORD) * (bi . biBitCount as DWORD)) }
};
}
