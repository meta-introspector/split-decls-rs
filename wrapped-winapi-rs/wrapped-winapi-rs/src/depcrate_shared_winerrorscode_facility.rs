// Generated macro for SCODE_FACILITY (function)
macro_rules! Depcrate_shared_winerrorSCODE_FACILITY {
() => {
// Module: crate::shared::winerror
// Provides: {"SCODE_FACILITY"}
// Dependencies: {}
# [inline] pub fn SCODE_FACILITY (sc : SCODE) -> HRESULT { (sc >> 16) & 0x1fff }
};
}
