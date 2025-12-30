// Generated macro for partial_take_until_incomplete (function)
macro_rules! Depcrate_token_testspartial_take_until_incomplete {
() => {
// Module: crate::token::tests
// Provides: {"partial_take_until_incomplete"}
// Dependencies: {}
# [test] fn partial_take_until_incomplete () { fn y < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { take_until (0 .. , "end") . parse_next (i) } assert_parse ! (y . parse_peek (Partial :: new (& b"nd" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (y . parse_peek (Partial :: new (& b"123" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (y . parse_peek (Partial :: new (& b"123en" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; }
};
}
