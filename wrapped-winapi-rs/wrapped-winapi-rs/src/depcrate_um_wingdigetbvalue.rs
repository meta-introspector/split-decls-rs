// Generated macro for GetBValue (function)
macro_rules! Depcrate_um_wingdiGetBValue {
() => {
// Module: crate::um::wingdi
// Provides: {"GetBValue"}
// Dependencies: {}
# [inline] pub fn GetBValue (rgb : COLORREF) -> BYTE { LOBYTE ((rgb >> 16) as WORD) }
};
}
