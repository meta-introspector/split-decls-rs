// Generated macro for MAKE_SCODE (function)
macro_rules! Depcrate_shared_winerrorMAKE_SCODE {
() => {
// Module: crate::shared::winerror
// Provides: {"MAKE_SCODE"}
// Dependencies: {}
# [inline] pub fn MAKE_SCODE (sev : HRESULT , fac : HRESULT , code : HRESULT) -> SCODE { (sev << 31) | (fac << 16) | code }
};
}
