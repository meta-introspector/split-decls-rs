// Generated macro for IS_ERROR (function)
macro_rules! Depcrate_shared_winerrorIS_ERROR {
() => {
// Module: crate::shared::winerror
// Provides: {"IS_ERROR"}
// Dependencies: {}
# [inline] pub fn IS_ERROR (hr : HRESULT) -> bool { (hr as u32) >> 31 == (SEVERITY_ERROR as u32) }
};
}
