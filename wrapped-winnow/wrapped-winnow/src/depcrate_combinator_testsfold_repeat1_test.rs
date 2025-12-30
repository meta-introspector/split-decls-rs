// Generated macro for fold_repeat1_test (function)
macro_rules! Depcrate_combinator_testsfold_repeat1_test {
() => {
// Module: crate::combinator::tests
// Provides: {"fold_repeat1_test"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn fold_repeat1_test () { fn fold_into_vec < T > (mut acc : Vec < T > , item : T) -> Vec < T > { acc . push (item) ; acc } fn multi < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , Vec < & 'i [u8] > > { repeat (1 .. , "abcd") . fold (Vec :: new , fold_into_vec) . parse_next (i) } let a = & b"abcdef" [..] ; let b = & b"abcdabcdefgh" [..] ; let c = & b"azerty" [..] ; let d = & b"abcdab" [..] ; assert_parse ! (multi . parse_peek (Partial :: new (a)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
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
                100,
            ],
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek (Partial :: new (b)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
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
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    97,
                    122,
                    101,
                    114,
                    116,
                    121,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek (Partial :: new (d)) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; }
};
}
