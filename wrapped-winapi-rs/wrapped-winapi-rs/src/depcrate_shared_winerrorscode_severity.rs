// Generated macro for SCODE_SEVERITY (function)
macro_rules! Depcrate_shared_winerrorSCODE_SEVERITY {
() => {
// Module: crate::shared::winerror
// Provides: {"SCODE_SEVERITY"}
// Dependencies: {}
# [inline] pub fn SCODE_SEVERITY (sc : SCODE) -> HRESULT { (sc >> 31) & 0x1 }
};
}
