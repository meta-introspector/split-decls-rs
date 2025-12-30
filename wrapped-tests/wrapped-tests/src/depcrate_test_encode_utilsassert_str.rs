// Generated macro for assert_str (function)
macro_rules! Depcrate_test_encode_utilsassert_str {
() => {
// Module: crate::test_encode_utils
// Provides: {"assert_str"}
// Dependencies: {}
# [allow (unused)] unsafe fn assert_str < T : Display > (s : * const c_char , expected : T) { let s = unsafe { CStr :: from_ptr (s) } . to_str () . unwrap () ; assert_eq ! (s , expected . to_string ()) ; }
};
}
