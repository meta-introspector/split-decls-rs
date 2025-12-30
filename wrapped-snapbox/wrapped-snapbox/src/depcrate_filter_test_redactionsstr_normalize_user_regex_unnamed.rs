// Generated macro for str_normalize_user_regex_unnamed (function)
macro_rules! Depcrate_filter_test_redactionsstr_normalize_user_regex_unnamed {
() => {
// Module: crate::filter::test_redactions
// Provides: {"str_normalize_user_regex_unnamed"}
// Dependencies: {}
# [test] # [cfg (feature = "regex")] fn str_normalize_user_regex_unnamed () { let input = "Hello world!" ; let pattern = "Hello [OBJECT]!" ; let mut sub = Redactions :: new () ; sub . insert ("[OBJECT]" , regex :: Regex :: new ("world") . unwrap ()) . unwrap () ; let actual = NormalizeToExpected :: new () . redact_with (& sub) . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , pattern . into_data ()) ; }
};
}
