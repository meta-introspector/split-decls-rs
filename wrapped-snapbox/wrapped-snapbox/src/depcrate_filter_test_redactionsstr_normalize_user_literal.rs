// Generated macro for str_normalize_user_literal (function)
macro_rules! Depcrate_filter_test_redactionsstr_normalize_user_literal {
() => {
// Module: crate::filter::test_redactions
// Provides: {"str_normalize_user_literal"}
// Dependencies: {}
# [test] fn str_normalize_user_literal () { let input = "Hello world!" ; let pattern = "Hello [OBJECT]!" ; let mut sub = Redactions :: new () ; sub . insert ("[OBJECT]" , "world") . unwrap () ; let actual = NormalizeToExpected :: new () . redact_with (& sub) . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , pattern . into_data ()) ; }
};
}
