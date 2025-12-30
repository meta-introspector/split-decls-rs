// Generated macro for terminated_test (function)
macro_rules! Depcrate_combinator_teststerminated_test {
() => {
// Module: crate::combinator::tests
// Provides: {"terminated_test"}
// Dependencies: {}
# [test] fn terminated_test () { fn terminated_abcd_efgh < 'i > (i : & mut Partial < & 'i [u8] > ,) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { terminated ("abcd" , "efgh") . parse_next (i) } assert_parse ! (terminated_abcd_efgh . parse_peek (Partial :: new (& b"abcdefghijkl" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                105,
                106,
                107,
                108,
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

"#]] . raw ()) ; assert_parse ! (terminated_abcd_efgh . parse_peek (Partial :: new (& b"ab" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (terminated_abcd_efgh . parse_peek (Partial :: new (& b"abcde" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (terminated_abcd_efgh . parse_peek (Partial :: new (& b"xxx" [..])) , str ! [[r#"
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

"#]] . raw ()) ; assert_parse ! (terminated_abcd_efgh . parse_peek (Partial :: new (& b"xxxxdef" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    120,
                    120,
                    120,
                    120,
                    100,
                    101,
                    102,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (terminated_abcd_efgh . parse_peek (Partial :: new (& b"abcdxxxx" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    120,
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
