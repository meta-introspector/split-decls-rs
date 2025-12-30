// Generated macro for complete_take_while_m_n_utf8_all_matching (function)
macro_rules! Depcrate_token_testscomplete_take_while_m_n_utf8_all_matching {
() => {
// Module: crate::token::tests
// Provides: {"complete_take_while_m_n_utf8_all_matching"}
// Dependencies: {}
# [test] fn complete_take_while_m_n_utf8_all_matching () { assert_parse ! (take_while (1 ..= 4 , | c : char | c . is_alphabetic ()) . parse_peek ("øn") , str ! [[r#"
Ok(
    (
        "",
        "øn",
    ),
)

"#]] . raw ()) ; }
};
}
