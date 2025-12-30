// Generated macro for partial_take_while_m_n_utf8_fixed (function)
macro_rules! Depcrate_token_testspartial_take_while_m_n_utf8_fixed {
() => {
// Module: crate::token::tests
// Provides: {"partial_take_while_m_n_utf8_fixed"}
// Dependencies: {}
# [test] fn partial_take_while_m_n_utf8_fixed () { fn parser < 'i > (i : & mut Partial < & 'i str >) -> TestResult < Partial < & 'i str > , & 'i str > { take_while (1 , | c | c == 'A' || c == '😃') . parse_next (i) } assert_parse ! (parser . parse_peek (Partial :: new ("A!")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "!",
            partial: true,
        },
        "A",
    ),
)

"#]] . raw ()) ; assert_parse ! (parser . parse_peek (Partial :: new ("😃!")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "!",
            partial: true,
        },
        "😃",
    ),
)

"#]] . raw ()) ; }
};
}
