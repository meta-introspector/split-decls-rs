// Generated macro for partial_take_utf8 (function)
macro_rules! Depcrate_token_testspartial_take_utf8 {
() => {
// Module: crate::token::tests
// Provides: {"partial_take_utf8"}
// Dependencies: {}
# [test] fn partial_take_utf8 () { fn f < 'i > (i : & mut Partial < & 'i str >) -> TestResult < Partial < & 'i str > , & 'i str > { take (3_usize) . parse_next (i) } assert_parse ! (f . parse_peek (Partial :: new ("")) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (f . parse_peek (Partial :: new ("ab")) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (f . parse_peek (Partial :: new ("點")) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (f . parse_peek (Partial :: new ("ab點cd")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "cd",
            partial: true,
        },
        "ab點",
    ),
)

"#]] . raw ()) ; assert_parse ! (f . parse_peek (Partial :: new ("a點bcd")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "cd",
            partial: true,
        },
        "a點b",
    ),
)

"#]] . raw ()) ; assert_parse ! (f . parse_peek (Partial :: new ("a點b")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "",
            partial: true,
        },
        "a點b",
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
