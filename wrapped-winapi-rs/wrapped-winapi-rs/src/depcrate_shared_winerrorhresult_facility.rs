// Generated macro for HRESULT_FACILITY (function)
macro_rules! Depcrate_shared_winerrorHRESULT_FACILITY {
() => {
// Module: crate::shared::winerror
// Provides: {"HRESULT_FACILITY"}
// Dependencies: {}
# [inline] pub fn HRESULT_FACILITY (hr : HRESULT) -> HRESULT { (hr >> 16) & 0x1fff }
};
}
