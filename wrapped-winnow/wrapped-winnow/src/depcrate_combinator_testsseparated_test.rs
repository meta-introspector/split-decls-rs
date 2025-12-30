// Generated macro for separated_test (function)
macro_rules! Depcrate_combinator_testsseparated_test {
() => {
// Module: crate::combinator::tests
// Provides: {"separated_test"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn separated_test () { fn multi < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , Vec < & 'i [u8] > > { separated (2 ..= 4 , "abcd" , ",") . parse_next (i) } let a = & b"abcd,ef" [..] ; let b = & b"abcd,abcd,efgh" [..] ; let c = & b"abcd,abcd,abcd,abcd,efgh" [..] ; let d = & b"abcd,abcd,abcd,abcd,abcd,efgh" [..] ; let e = & b"abcd,ab" [..] ; assert_parse ! (multi . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    101,
                    102,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek (Partial :: new (b)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                44,
                101,
                102,
                103,
                104,
            ],
            partial: true,
        },
        [
            [
                97,
                98,
                99,
                100,
            ],
            [
                97,
                98,
                99,
                100,
            ],
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek (Partial :: new (c)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                44,
                101,
                102,
                103,
                104,
            ],
            partial: true,
        },
        [
            [
                97,
                98,
                99,
                100,
            ],
            [
                97,
                98,
                99,
                100,
            ],
            [
                97,
                98,
                99,
                100,
            ],
            [
                97,
                98,
                99,
                100,
            ],
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek (Partial :: new (d)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                44,
                97,
                98,
                99,
                100,
                44,
                101,
                102,
                103,
                104,
            ],
            partial: true,
        },
        [
            [
                97,
                98,
                99,
                100,
            ],
            [
                97,
                98,
                99,
                100,
            ],
            [
                97,
                98,
                99,
                100,
            ],
            [
                97,
                98,
                99,
                100,
            ],
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek (Partial :: new (e)) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; }
};
}
