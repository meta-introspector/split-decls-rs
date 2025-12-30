// Generated macro for HIWORD (function)
macro_rules! Depcrate_shared_minwindefHIWORD {
() => {
// Module: crate::shared::minwindef
// Provides: {"HIWORD"}
// Dependencies: {}
# [inline] pub fn HIWORD (l : DWORD) -> WORD { ((l >> 16) & 0xffff) as WORD }
};
}
