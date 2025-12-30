// Generated macro for preceded_test (function)
macro_rules! Depcrate_combinator_testspreceded_test {
() => {
// Module: crate::combinator::tests
// Provides: {"preceded_test"}
// Dependencies: {}
# [test] fn preceded_test () { fn preceded_abcd_efgh < 'i > (i : & mut Partial < & 'i [u8] > ,) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { preceded ("abcd" , "efgh") . parse_next (i) } assert_parse ! (preceded_abcd_efgh . parse_peek (Partial :: new (& b"abcdefghijkl" [..])) , str ! [[r#"
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
            101,
            102,
            103,
            104,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (preceded_abcd_efgh . parse_peek (Partial :: new (& b"ab" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (preceded_abcd_efgh . parse_peek (Partial :: new (& b"abcde" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (preceded_abcd_efgh . parse_peek (Partial :: new (& b"xxx" [..])) , str ! [[r#"
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

"#]] . raw ()) ; assert_parse ! (preceded_abcd_efgh . parse_peek (Partial :: new (& b"xxxxdef" [..])) , str ! [[r#"
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

"#]] . raw ()) ; assert_parse ! (preceded_abcd_efgh . parse_peek (Partial :: new (& b"abcdxxx" [..])) , str ! [[r#"
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
