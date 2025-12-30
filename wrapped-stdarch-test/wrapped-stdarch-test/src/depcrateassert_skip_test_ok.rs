// Generated macro for assert_skip_test_ok (function)
macro_rules! Depcrateassert_skip_test_ok {
() => {
// Module: crate
// Provides: {"assert_skip_test_ok"}
// Dependencies: {}
pub fn assert_skip_test_ok (name : & str , missing_features : & [& str]) { println ! ("Skipping test `{name}` due to missing target features:") ; for feature in missing_features { println ! ("  - {feature}") ; } match env :: var ("STDARCH_TEST_EVERYTHING") { Ok (_) => panic ! ("skipped test `{name}` when it shouldn't be skipped") , Err (_) => println ! ("Set STDARCH_TEST_EVERYTHING to make this an error.") , } }
};
}
