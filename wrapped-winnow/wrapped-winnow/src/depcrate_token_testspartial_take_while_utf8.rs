// Generated macro for partial_take_while_utf8 (function)
macro_rules! Depcrate_token_testspartial_take_while_utf8 {
() => {
// Module: crate::token::tests
// Provides: {"partial_take_while_utf8"}
// Dependencies: {}
# [test] fn partial_take_while_utf8 () { fn f < 'i > (i : & mut Partial < & 'i str >) -> TestResult < Partial < & 'i str > , & 'i str > { take_while (0 .. , | c | c != '點') . parse_next (i) } assert_parse ! (f . parse_peek (Partial :: new ("")) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (f . parse_peek (Partial :: new ("abcd")) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (f . parse_peek (Partial :: new ("abcd點")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "點",
            partial: true,
        },
        "abcd",
    ),
)

"#]] . raw ()) ; assert_parse ! (f . parse_peek (Partial :: new ("abcd點a")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "點a",
            partial: true,
        },
        "abcd",
    ),
)

"#]] . raw ()) ; fn g < 'i > (i : & mut Partial < & 'i str >) -> TestResult < Partial < & 'i str > , & 'i str > { take_while (0 .. , | c | c == '點') . parse_next (i) } assert_parse ! (g . parse_peek (Partial :: new ("")) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (g . parse_peek (Partial :: new ("點abcd")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "abcd",
            partial: true,
        },
        "點",
    ),
)

"#]] . raw ()) ; assert_parse ! (g . parse_peek (Partial :: new ("點點點a")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "a",
            partial: true,
        },
        "點點點",
    ),
)

"#]] . raw ()) ; }
};
}
