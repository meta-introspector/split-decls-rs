// Generated macro for complete_take_while_m_n_utf8_all_matching_substring (function)
macro_rules! Depcrate_token_testscomplete_take_while_m_n_utf8_all_matching_substring {
() => {
// Module: crate::token::tests
// Provides: {"complete_take_while_m_n_utf8_all_matching_substring"}
// Dependencies: {}
# [test] fn complete_take_while_m_n_utf8_all_matching_substring () { assert_parse ! (take_while (1 , | c : char | c . is_alphabetic ()) . parse_peek ("øn") , str ! [[r#"
Ok(
    (
        "n",
        "ø",
    ),
)

"#]] . raw ()) ; }
};
}
