// Generated macro for MAKELONG (function)
macro_rules! Depcrate_shared_minwindefMAKELONG {
() => {
// Module: crate::shared::minwindef
// Provides: {"MAKELONG"}
// Dependencies: {}
# [inline] pub fn MAKELONG (a : WORD , b : WORD) -> LONG { ((a as DWORD) | ((b as DWORD) << 16)) as LONG }
};
}
