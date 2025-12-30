// Generated macro for MAKE_SCODE (macro)
macro_rules! Depcrate_macrosMAKE_SCODE {
() => {
// Module: crate::macros
// Provides: {"MAKE_SCODE"}
// Dependencies: {}
macro_rules ! MAKE_SCODE { ($ sev : expr , $ fac : expr , $ code : expr) => { ($ sev << 31) | ($ fac << 16) | $ code } }
};
}
