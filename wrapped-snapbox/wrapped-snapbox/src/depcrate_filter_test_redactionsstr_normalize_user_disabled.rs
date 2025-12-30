// Generated macro for str_normalize_user_disabled (function)
macro_rules! Depcrate_filter_test_redactionsstr_normalize_user_disabled {
() => {
// Module: crate::filter::test_redactions
// Provides: {"str_normalize_user_disabled"}
// Dependencies: {}
# [test] fn str_normalize_user_disabled () { let input = "cargo" ; let pattern = "cargo[EXE]" ; let mut sub = Redactions :: new () ; sub . insert ("[EXE]" , "") . unwrap () ; let actual = NormalizeToExpected :: new () . redact_with (& sub) . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , pattern . into_data ()) ; }
};
}
