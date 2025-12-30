// Generated macro for GetGValue (function)
macro_rules! Depcrate_um_wingdiGetGValue {
() => {
// Module: crate::um::wingdi
// Provides: {"GetGValue"}
// Dependencies: {}
# [inline] pub fn GetGValue (rgb : COLORREF) -> BYTE { LOBYTE ((rgb as WORD) >> 8) }
};
}
