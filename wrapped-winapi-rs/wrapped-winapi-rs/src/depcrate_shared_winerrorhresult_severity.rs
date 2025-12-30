// Generated macro for HRESULT_SEVERITY (function)
macro_rules! Depcrate_shared_winerrorHRESULT_SEVERITY {
() => {
// Module: crate::shared::winerror
// Provides: {"HRESULT_SEVERITY"}
// Dependencies: {}
# [inline] pub fn HRESULT_SEVERITY (hr : HRESULT) -> HRESULT { (hr >> 31) & 0x1 }
};
}
