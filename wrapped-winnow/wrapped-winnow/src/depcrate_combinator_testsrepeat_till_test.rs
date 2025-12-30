// Generated macro for repeat_till_test (function)
macro_rules! Depcrate_combinator_testsrepeat_till_test {
() => {
// Module: crate::combinator::tests
// Provides: {"repeat_till_test"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn repeat_till_test () { # [allow (clippy :: type_complexity)] fn multi < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , (Vec < & 'i [u8] > , & 'i [u8]) > { repeat_till (0 .. , "abcd" , "efgh") . parse_next (i) } let a = b"abcdabcdefghabcd" ; let b = b"efghabcd" ; let c = b"azerty" ; assert_parse ! (multi . parse_peek (& a [..]) , str ! [[r#"
Ok(
    (
        [
            97,
            98,
            99,
            100,
        ],
        (
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
            [
                101,
                102,
                103,
                104,
            ],
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek (& b [..]) , str ! [[r#"
Ok(
    (
        [
            97,
            98,
            99,
            100,
        ],
        (
            [],
            [
                101,
                102,
                103,
                104,
            ],
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek (& c [..]) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: [
                97,
                122,
                101,
                114,
                116,
                121,
            ],
        },
    ),
)

"#]] . raw ()) ; }
};
}
