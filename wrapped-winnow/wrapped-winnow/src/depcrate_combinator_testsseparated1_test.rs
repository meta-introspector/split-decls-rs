// Generated macro for separated1_test (function)
macro_rules! Depcrate_combinator_testsseparated1_test {
() => {
// Module: crate::combinator::tests
// Provides: {"separated1_test"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn separated1_test () { fn multi < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , Vec < & 'i [u8] > > { separated (1 .. , "abcd" , ",") . parse_next (i) } fn multi_longsep < 'i > (i : & mut Partial < & 'i [u8] > ,) -> TestResult < Partial < & 'i [u8] > , Vec < & 'i [u8] > > { separated (1 .. , "abcd" , "..") . parse_next (i) } let a = & b"abcdef" [..] ; let b = & b"abcd,abcdef" [..] ; let c = & b"azerty" [..] ; let d = & b"abcd,abcd,ef" [..] ; let f = & b"abc" [..] ; let g = & b"abcd." [..] ; let h = & b"abcd,abc" [..] ; assert_parse ! (multi . parse_peek (Partial :: new (a)) , str ! [[r#"
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
Ok(
    (
        Partial {
            input: [
                44,
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
            [
                97,
                98,
                99,
                100,
            ],
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek (Partial :: new (f)) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (multi_longsep . parse_peek (Partial :: new (g)) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek (Partial :: new (h)) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; }
};
}
