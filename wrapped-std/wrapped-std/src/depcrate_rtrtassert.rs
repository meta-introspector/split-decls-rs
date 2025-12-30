// Generated macro for rtassert (macro)
macro_rules! Depcrate_rtrtassert {
() => {
// Module: crate::rt
// Provides: {"rtassert"}
// Dependencies: {}
macro_rules ! rtassert { ($ e : expr) => { if !$ e { rtabort ! (concat ! ("assertion failed: " , stringify ! ($ e))) ; } } ; }
};
}
