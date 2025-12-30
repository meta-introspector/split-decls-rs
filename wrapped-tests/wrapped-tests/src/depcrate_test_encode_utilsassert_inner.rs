// Generated macro for assert_inner (macro)
macro_rules! Depcrate_test_encode_utilsassert_inner {
() => {
// Module: crate::test_encode_utils
// Provides: {"assert_inner"}
// Dependencies: {}
macro_rules ! assert_inner { (no_atomic $ ($ t : tt) *) => { } ; (enc $ (# [$ m : meta]) * $ stat : ident => $ expected : expr) => { $ (# [$ m]) * # [test] fn $ stat () { extern "C" { static $ stat : * const c_char ; } unsafe { assert_encoding ($ stat , $ expected) } ; } } ; (str $ (# [$ m : meta]) * $ stat : ident => $ expected : expr) => { $ (# [$ m]) * # [test] fn $ stat () { extern "C" { static $ stat : * const c_char ; } unsafe { assert_str ($ stat , $ expected) } ; } } ; }
};
}
