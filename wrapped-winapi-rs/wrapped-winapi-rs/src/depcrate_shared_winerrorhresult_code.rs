// Generated macro for HRESULT_CODE (function)
macro_rules! Depcrate_shared_winerrorHRESULT_CODE {
() => {
// Module: crate::shared::winerror
// Provides: {"HRESULT_CODE"}
// Dependencies: {}
# [inline] pub fn HRESULT_CODE (hr : HRESULT) -> HRESULT { hr & 0xFFFF }
};
}
