// Generated macro for peek_test (function)
macro_rules! Depcrate_combinator_testspeek_test {
() => {
// Module: crate::combinator::tests
// Provides: {"peek_test"}
// Dependencies: {}
# [test] fn peek_test () { fn peek_literal < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { peek ("abcd") . parse_next (i) } assert_parse ! (peek_literal . parse_peek (Partial :: new (& b"abcdef" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                97,
                98,
                99,
                100,
                101,
                102,
            ],
            partial: true,
        },
        [
            97,
            98,
            99,
            100,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (peek_literal . parse_peek (Partial :: new (& b"ab" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (peek_literal . parse_peek (Partial :: new (& b"xxx" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    120,
                    120,
                    120,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; }
};
}
