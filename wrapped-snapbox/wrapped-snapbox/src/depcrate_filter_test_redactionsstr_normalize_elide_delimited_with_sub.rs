// Generated macro for str_normalize_elide_delimited_with_sub (function)
macro_rules! Depcrate_filter_test_redactionsstr_normalize_elide_delimited_with_sub {
() => {
// Module: crate::filter::test_redactions
// Provides: {"str_normalize_elide_delimited_with_sub"}
// Dependencies: {}
# [test] fn str_normalize_elide_delimited_with_sub () { let input = "Hello World\nHow are you?\nGoodbye World" ; let pattern = "Hello [..]\n...\nGoodbye [..]" ; let expected = "Hello [..]\n...\nGoodbye [..]" ; let actual = NormalizeToExpected :: new () . redact () . normalize (input . into () , & pattern . into ()) ; assert_eq ! (actual , expected . into_data ()) ; }
};
}
