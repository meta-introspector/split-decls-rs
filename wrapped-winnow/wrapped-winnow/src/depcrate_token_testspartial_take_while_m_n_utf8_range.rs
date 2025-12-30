// Generated macro for partial_take_while_m_n_utf8_range (function)
macro_rules! Depcrate_token_testspartial_take_while_m_n_utf8_range {
() => {
// Module: crate::token::tests
// Provides: {"partial_take_while_m_n_utf8_range"}
// Dependencies: {}
# [test] fn partial_take_while_m_n_utf8_range () { fn parser < 'i > (i : & mut Partial < & 'i str >) -> TestResult < Partial < & 'i str > , & 'i str > { take_while (1 ..= 2 , | c | c == 'A' || c == '😃') . parse_next (i) } assert_parse ! (parser . parse_peek (Partial :: new ("A!")) , str ! [[r#"
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
