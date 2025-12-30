// Generated macro for impl_zeroize_with_default (macro)
macro_rules! Depcrateimpl_zeroize_with_default {
() => {
// Module: crate
// Provides: {"impl_zeroize_with_default"}
// Dependencies: {}
macro_rules ! impl_zeroize_with_default { ($ ($ type : ty) ,+) => { $ (impl DefaultIsZeroes for $ type { }) + } ; }
};
}
