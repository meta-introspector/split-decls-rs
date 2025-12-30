// Generated macro for count_test (function)
macro_rules! Depcrate_combinator_testscount_test {
() => {
// Module: crate::combinator::tests
// Provides: {"count_test"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn count_test () { const TIMES : usize = 2 ; fn cnt_2 < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , Vec < & 'i [u8] > > { repeat (TIMES , "abc") . parse_next (i) } assert_parse ! (cnt_2 . parse_peek (Partial :: new (& b"abcabcabcdef" [..])) , str ! [[r#"
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
            [
                97,
                98,
                99,
            ],
            [
                97,
                98,
                99,
            ],
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (cnt_2 . parse_peek (Partial :: new (& b"ab" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (cnt_2 . parse_peek (Partial :: new (& b"abcab" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (cnt_2 . parse_peek (Partial :: new (& b"xxx" [..])) , str ! [[r#"
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

"#]] . raw ()) ; assert_parse ! (cnt_2 . parse_peek (Partial :: new (& b"xxxabcabcdef" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    120,
                    120,
                    120,
                    97,
                    98,
                    99,
                    97,
                    98,
                    99,
                    100,
                    101,
                    102,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (cnt_2 . parse_peek (Partial :: new (& b"abcxxxabcdef" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    120,
                    120,
                    120,
                    97,
                    98,
                    99,
                    100,
                    101,
                    102,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; }
};
}
