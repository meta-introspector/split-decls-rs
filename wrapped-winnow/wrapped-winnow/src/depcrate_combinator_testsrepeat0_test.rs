// Generated macro for repeat0_test (function)
macro_rules! Depcrate_combinator_testsrepeat0_test {
() => {
// Module: crate::combinator::tests
// Provides: {"repeat0_test"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn repeat0_test () { fn multi < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , Vec < & 'i [u8] > > { repeat (0 .. , "abcd") . parse_next (i) } assert_parse ! (multi . parse_peek (Partial :: new (& b"abcdef" [..])) , str ! [[r#"
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

"#]] . raw ()) ; assert_parse ! (multi . parse_peek (Partial :: new (& b"abcdabcdefgh" [..])) , str ! [[r#"
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

"#]] . raw ()) ; assert_parse ! (multi . parse_peek (Partial :: new (& b"azerty" [..])) , str ! [[r#"
Ok(
    (
        Partial {
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
        [],
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek (Partial :: new (& b"abcdab" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek (Partial :: new (& b"abcd" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek (Partial :: new (& b"" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; }
};
}
