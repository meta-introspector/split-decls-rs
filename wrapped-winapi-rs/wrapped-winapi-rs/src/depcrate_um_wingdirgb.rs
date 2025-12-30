// Generated macro for RGB (function)
macro_rules! Depcrate_um_wingdiRGB {
() => {
// Module: crate::um::wingdi
// Provides: {"RGB"}
// Dependencies: {}
# [inline] pub fn RGB (r : BYTE , g : BYTE , b : BYTE) -> COLORREF { r as COLORREF | ((g as COLORREF) << 8) | ((b as COLORREF) << 16) }
};
}
