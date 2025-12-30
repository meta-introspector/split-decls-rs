// Generated macro for separated_pair_test (function)
macro_rules! Depcrate_combinator_testsseparated_pair_test {
() => {
// Module: crate::combinator::tests
// Provides: {"separated_pair_test"}
// Dependencies: {}
# [test] fn separated_pair_test () { # [allow (clippy :: type_complexity)] fn sep_pair_abc_def < 'i > (i : & mut Partial < & 'i [u8] > ,) -> TestResult < Partial < & 'i [u8] > , (& 'i [u8] , & 'i [u8]) > { separated_pair ("abc" , "," , "def") . parse_next (i) } assert_parse ! (sep_pair_abc_def . parse_peek (Partial :: new (& b"abc,defghijkl" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                103,
                104,
                105,
                106,
                107,
                108,
            ],
            partial: true,
        },
        (
            [
                97,
                98,
                99,
            ],
            [
                100,
                101,
                102,
            ],
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (sep_pair_abc_def . parse_peek (Partial :: new (& b"ab" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (sep_pair_abc_def . parse_peek (Partial :: new (& b"abc,d" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (sep_pair_abc_def . parse_peek (Partial :: new (& b"xxx" [..])) , str ! [[r#"
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

"#]] . raw ()) ; assert_parse ! (sep_pair_abc_def . parse_peek (Partial :: new (& b"xxx,def" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    120,
                    120,
                    120,
                    44,
                    100,
                    101,
                    102,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (sep_pair_abc_def . parse_peek (Partial :: new (& b"abc,xxx" [..])) , str ! [[r#"
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
