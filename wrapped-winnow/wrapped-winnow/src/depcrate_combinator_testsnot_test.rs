// Generated macro for not_test (function)
macro_rules! Depcrate_combinator_testsnot_test {
() => {
// Module: crate::combinator::tests
// Provides: {"not_test"}
// Dependencies: {}
# [test] fn not_test () { fn not_aaa < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , () > { not ("aaa") . parse_next (i) } assert_parse ! (not_aaa . parse_peek (Partial :: new (& b"aaa" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    97,
                    97,
                    97,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (not_aaa . parse_peek (Partial :: new (& b"aa" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (not_aaa . parse_peek (Partial :: new (& b"abcd" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                97,
                98,
                99,
                100,
            ],
            partial: true,
        },
        (),
    ),
)

"#]] . raw ()) ; }
};
}
