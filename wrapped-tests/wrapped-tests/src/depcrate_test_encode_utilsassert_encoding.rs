// Generated macro for assert_encoding (function)
macro_rules! Depcrate_test_encode_utilsassert_encoding {
() => {
// Module: crate::test_encode_utils
// Provides: {"assert_encoding"}
// Dependencies: {}
unsafe fn assert_encoding (s : * const c_char , e : Encoding) { let s = unsafe { CStr :: from_ptr (s) } . to_str () . unwrap () ; if ! e . equivalent_to_str (s) { panic ! ("{} were not equivalent to {}" , e , s) ; } assert_eq ! (e . to_string () , s . trim_start_matches ('r')) ; }
};
}
