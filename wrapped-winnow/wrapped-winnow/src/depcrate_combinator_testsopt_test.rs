// Generated macro for opt_test (function)
macro_rules! Depcrate_combinator_testsopt_test {
() => {
// Module: crate::combinator::tests
// Provides: {"opt_test"}
// Dependencies: {}
# [test] fn opt_test () { fn opt_abcd < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , Option < & 'i [u8] > > { opt ("abcd") . parse_next (i) } let a = & b"abcdef" [..] ; let b = & b"bcdefg" [..] ; let c = & b"ab" [..] ; assert_parse ! (opt_abcd . parse_peek (Partial :: new (a)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                101,
                102,
            ],
            partial: true,
        },
        Some(
            [
                97,
                98,
                99,
                100,
            ],
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (opt_abcd . parse_peek (Partial :: new (b)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                98,
                99,
                100,
                101,
                102,
                103,
            ],
            partial: true,
        },
        None,
    ),
)

"#]] . raw ()) ; assert_parse ! (opt_abcd . parse_peek (Partial :: new (c)) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; }
};
}
