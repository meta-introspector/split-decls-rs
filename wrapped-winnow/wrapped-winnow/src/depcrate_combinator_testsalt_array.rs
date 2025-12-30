// Generated macro for alt_array (function)
macro_rules! Depcrate_combinator_testsalt_array {
() => {
// Module: crate::combinator::tests
// Provides: {"alt_array"}
// Dependencies: {}
# [test] fn alt_array () { fn alt1 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , & 'i [u8] > { alt (["a" , "bc" , "def"]) . parse_next (i) } let i = & b"a" [..] ; assert_parse ! (alt1 . parse_peek (i) , str ! [[r#"
Ok(
    (
        [],
        [
            97,
        ],
    ),
)

"#]] . raw ()) ; let i = & b"bc" [..] ; assert_parse ! (alt1 . parse_peek (i) , str ! [[r#"
Ok(
    (
        [],
        [
            98,
            99,
        ],
    ),
)

"#]] . raw ()) ; let i = & b"defg" [..] ; assert_parse ! (alt1 . parse_peek (i) , str ! [[r#"
Ok(
    (
        [
            103,
        ],
        [
            100,
            101,
            102,
        ],
    ),
)

"#]] . raw ()) ; let i = & b"z" [..] ; assert_parse ! (alt1 . parse_peek (i) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: [
                122,
            ],
        },
    ),
)

"#]] . raw ()) ; }
};
}
