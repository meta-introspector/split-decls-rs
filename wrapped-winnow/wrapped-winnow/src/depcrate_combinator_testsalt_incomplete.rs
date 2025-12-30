// Generated macro for alt_incomplete (function)
macro_rules! Depcrate_combinator_testsalt_incomplete {
() => {
// Module: crate::combinator::tests
// Provides: {"alt_incomplete"}
// Dependencies: {}
# [test] fn alt_incomplete () { fn alt1 < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { alt (("a" , "bc" , "def")) . parse_next (i) } let a = & b"" [..] ; assert_parse ! (alt1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; let a = & b"b" [..] ; assert_parse ! (alt1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; let a = & b"bcd" [..] ; assert_parse ! (alt1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                100,
            ],
            partial: true,
        },
        [
            98,
            99,
        ],
    ),
)

"#]] . raw ()) ; let a = & b"cde" [..] ; assert_parse ! (alt1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    99,
                    100,
                    101,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; let a = & b"de" [..] ; assert_parse ! (alt1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; let a = & b"defg" [..] ; assert_parse ! (alt1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                103,
            ],
            partial: true,
        },
        [
            100,
            101,
            102,
        ],
    ),
)

"#]] . raw ()) ; }
};
}
