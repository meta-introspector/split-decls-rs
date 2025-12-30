// Generated macro for MAKEWORD (function)
macro_rules! Depcrate_shared_minwindefMAKEWORD {
() => {
// Module: crate::shared::minwindef
// Provides: {"MAKEWORD"}
// Dependencies: {}
# [inline] pub fn MAKEWORD (a : BYTE , b : BYTE) -> WORD { (a as WORD) | ((b as WORD) << 8) }
};
}
