// Generated macro for delimited_test (function)
macro_rules! Depcrate_combinator_testsdelimited_test {
() => {
// Module: crate::combinator::tests
// Provides: {"delimited_test"}
// Dependencies: {}
# [test] fn delimited_test () { fn delimited_abc_def_ghi < 'i > (i : & mut Partial < & 'i [u8] > ,) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { delimited ("abc" , "def" , "ghi") . parse_next (i) } assert_parse ! (delimited_abc_def_ghi . parse_peek (Partial :: new (& b"abcdefghijkl" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                106,
                107,
                108,
            ],
            partial: true,
        },
        [
            100,
            101,
            102,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (delimited_abc_def_ghi . parse_peek (Partial :: new (& b"ab" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (delimited_abc_def_ghi . parse_peek (Partial :: new (& b"abcde" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (delimited_abc_def_ghi . parse_peek (Partial :: new (& b"abcdefgh" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (delimited_abc_def_ghi . parse_peek (Partial :: new (& b"xxx" [..])) , str ! [[r#"
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

"#]] . raw ()) ; assert_parse ! (delimited_abc_def_ghi . parse_peek (Partial :: new (& b"xxxdefghi" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    120,
                    120,
                    120,
                    100,
                    101,
                    102,
                    103,
                    104,
                    105,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (delimited_abc_def_ghi . parse_peek (Partial :: new (& b"abcxxxghi" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    120,
                    120,
                    120,
                    103,
                    104,
                    105,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (delimited_abc_def_ghi . parse_peek (Partial :: new (& b"abcdefxxx" [..])) , str ! [[r#"
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
