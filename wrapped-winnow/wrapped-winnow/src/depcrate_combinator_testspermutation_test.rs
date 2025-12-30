// Generated macro for permutation_test (function)
macro_rules! Depcrate_combinator_testspermutation_test {
() => {
// Module: crate::combinator::tests
// Provides: {"permutation_test"}
// Dependencies: {}
# [test] fn permutation_test () { # [allow (clippy :: type_complexity)] fn perm < 'i > (i : & mut Partial < & 'i [u8] > ,) -> TestResult < Partial < & 'i [u8] > , (& 'i [u8] , & 'i [u8] , & 'i [u8]) > { permutation (("abcd" , "efg" , "hi")) . parse_next (i) } let a = & b"abcdefghijk" [..] ; assert_parse ! (perm . parse_peek (Partial :: new (a)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                106,
                107,
            ],
            partial: true,
        },
        (
            [
                97,
                98,
                99,
                100,
            ],
            [
                101,
                102,
                103,
            ],
            [
                104,
                105,
            ],
        ),
    ),
)

"#]] . raw ()) ; let b = & b"efgabcdhijk" [..] ; assert_parse ! (perm . parse_peek (Partial :: new (b)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                106,
                107,
            ],
            partial: true,
        },
        (
            [
                97,
                98,
                99,
                100,
            ],
            [
                101,
                102,
                103,
            ],
            [
                104,
                105,
            ],
        ),
    ),
)

"#]] . raw ()) ; let c = & b"hiefgabcdjk" [..] ; assert_parse ! (perm . parse_peek (Partial :: new (c)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                106,
                107,
            ],
            partial: true,
        },
        (
            [
                97,
                98,
                99,
                100,
            ],
            [
                101,
                102,
                103,
            ],
            [
                104,
                105,
            ],
        ),
    ),
)

"#]] . raw ()) ; let d = & b"efgxyzabcdefghi" [..] ; assert_parse ! (perm . parse_peek (Partial :: new (d)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    120,
                    121,
                    122,
                    97,
                    98,
                    99,
                    100,
                    101,
                    102,
                    103,
                    104,
                    105,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; let e = & b"efgabc" [..] ; assert_parse ! (perm . parse_peek (Partial :: new (e)) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; }
};
}
