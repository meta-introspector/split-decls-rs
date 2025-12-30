// Generated macro for GDI_DIBSIZE (function)
macro_rules! Depcrate_um_wingdiGDI_DIBSIZE {
() => {
// Module: crate::um::wingdi
// Provides: {"GDI_DIBSIZE"}
// Dependencies: {}
# [inline] pub fn GDI_DIBSIZE (bi : & BITMAPINFOHEADER) -> DWORD { if bi . biHeight < 0 { GDI__DIBSIZE (bi) * - 1i32 as u32 } else { GDI__DIBSIZE (bi) } }
};
}
