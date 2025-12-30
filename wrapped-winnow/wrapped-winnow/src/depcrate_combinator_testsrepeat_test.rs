// Generated macro for repeat_test (function)
macro_rules! Depcrate_combinator_testsrepeat_test {
() => {
// Module: crate::combinator::tests
// Provides: {"repeat_test"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn repeat_test () { fn multi < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , Vec < & 'i [u8] > > { repeat (2 ..= 4 , "Abcd") . parse_next (i) } let a = & b"Abcdef" [..] ; let b = & b"AbcdAbcdefgh" [..] ; let c = & b"AbcdAbcdAbcdAbcdefgh" [..] ; let d = & b"AbcdAbcdAbcdAbcdAbcdefgh" [..] ; let e = & b"AbcdAb" [..] ; assert_parse ! (multi . parse_peek (Partial :: new (a)) , str ! [[r#"
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
                101,
                102,
                103,
                104,
            ],
            partial: true,
        },
        [
            [
                65,
                98,
                99,
                100,
            ],
            [
                65,
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
                101,
                102,
                103,
                104,
            ],
            partial: true,
        },
        [
            [
                65,
                98,
                99,
                100,
            ],
            [
                65,
                98,
                99,
                100,
            ],
            [
                65,
                98,
                99,
                100,
            ],
            [
                65,
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
                65,
                98,
                99,
                100,
                101,
                102,
                103,
                104,
            ],
            partial: true,
        },
        [
            [
                65,
                98,
                99,
                100,
            ],
            [
                65,
                98,
                99,
                100,
            ],
            [
                65,
                98,
                99,
                100,
            ],
            [
                65,
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
