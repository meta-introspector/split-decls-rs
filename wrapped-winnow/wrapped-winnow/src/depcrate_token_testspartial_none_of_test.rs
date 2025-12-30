// Generated macro for partial_none_of_test (function)
macro_rules! Depcrate_token_testspartial_none_of_test {
() => {
// Module: crate::token::tests
// Provides: {"partial_none_of_test"}
// Dependencies: {}
# [test] fn partial_none_of_test () { fn f < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , u8 > { none_of (['a' , 'b']) . parse_next (i) } let a = & b"abcd" [..] ; assert_parse ! (f . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    97,
                    98,
                    99,
                    100,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; let b = & b"cde" [..] ; assert_parse ! (f . parse_peek (Partial :: new (b)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                100,
                101,
            ],
            partial: true,
        },
        99,
    ),
)

"#]] . raw ()) ; }
};
}
