// Generated macro for GDI__DIBSIZE (function)
macro_rules! Depcrate_um_wingdiGDI__DIBSIZE {
() => {
// Module: crate::um::wingdi
// Provides: {"GDI__DIBSIZE"}
// Dependencies: {}
# [inline] pub fn GDI__DIBSIZE (bi : & BITMAPINFOHEADER) -> DWORD { GDI_DIBWIDTHBYTES (bi) * bi . biHeight as DWORD }
};
}
