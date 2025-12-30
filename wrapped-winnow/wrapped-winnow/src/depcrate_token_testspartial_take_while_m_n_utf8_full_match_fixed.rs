// Generated macro for partial_take_while_m_n_utf8_full_match_fixed (function)
macro_rules! Depcrate_token_testspartial_take_while_m_n_utf8_full_match_fixed {
() => {
// Module: crate::token::tests
// Provides: {"partial_take_while_m_n_utf8_full_match_fixed"}
// Dependencies: {}
# [test] fn partial_take_while_m_n_utf8_full_match_fixed () { fn parser < 'i > (i : & mut Partial < & 'i str >) -> TestResult < Partial < & 'i str > , & 'i str > { take_while (1 , | c : char | c . is_alphabetic ()) . parse_next (i) } assert_parse ! (parser . parse_peek (Partial :: new ("øn")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "n",
            partial: true,
        },
        "ø",
    ),
)

"#]] . raw ()) ; }
};
}
