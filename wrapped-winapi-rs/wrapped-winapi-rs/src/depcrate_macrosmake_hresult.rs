// Generated macro for MAKE_HRESULT (macro)
macro_rules! Depcrate_macrosMAKE_HRESULT {
() => {
// Module: crate::macros
// Provides: {"MAKE_HRESULT"}
// Dependencies: {}
macro_rules ! MAKE_HRESULT { ($ sev : expr , $ fac : expr , $ code : expr) => { ($ sev << 31) | ($ fac << 16) | $ code } }
};
}
