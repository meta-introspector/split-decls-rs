// Generated macro for CMYK (function)
macro_rules! Depcrate_um_wingdiCMYK {
() => {
// Module: crate::um::wingdi
// Provides: {"CMYK"}
// Dependencies: {}
# [inline] pub fn CMYK (c : BYTE , m : BYTE , y : BYTE , k : BYTE) -> COLORREF { (k as COLORREF) | ((y as COLORREF) << 8) | ((m as COLORREF) << 16) | ((c as COLORREF) << 24) }
};
}
