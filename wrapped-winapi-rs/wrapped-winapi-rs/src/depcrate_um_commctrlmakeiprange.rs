// Generated macro for MAKEIPRANGE (function)
macro_rules! Depcrate_um_commctrlMAKEIPRANGE {
() => {
// Module: crate::um::commctrl
// Provides: {"MAKEIPRANGE"}
// Dependencies: {}
# [inline] pub fn MAKEIPRANGE (low : BYTE , high : BYTE) -> LPARAM { (((high as WORD) << 8) + low as WORD) as LPARAM }
};
}
