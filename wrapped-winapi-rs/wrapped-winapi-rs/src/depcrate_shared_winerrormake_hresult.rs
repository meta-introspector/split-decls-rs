// Generated macro for MAKE_HRESULT (function)
macro_rules! Depcrate_shared_winerrorMAKE_HRESULT {
() => {
// Module: crate::shared::winerror
// Provides: {"MAKE_HRESULT"}
// Dependencies: {}
# [inline] pub fn MAKE_HRESULT (sev : HRESULT , fac : HRESULT , code : HRESULT) -> HRESULT { (sev << 31) | (fac << 16) | code }
};
}
