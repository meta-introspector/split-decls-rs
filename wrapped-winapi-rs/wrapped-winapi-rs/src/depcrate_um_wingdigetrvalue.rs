// Generated macro for GetRValue (function)
macro_rules! Depcrate_um_wingdiGetRValue {
() => {
// Module: crate::um::wingdi
// Provides: {"GetRValue"}
// Dependencies: {}
# [inline] pub fn GetRValue (rgb : COLORREF) -> BYTE { LOBYTE (rgb as WORD) }
};
}
