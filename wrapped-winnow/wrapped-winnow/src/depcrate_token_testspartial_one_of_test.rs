// Generated macro for partial_one_of_test (function)
macro_rules! Depcrate_token_testspartial_one_of_test {
() => {
// Module: crate::token::tests
// Provides: {"partial_one_of_test"}
// Dependencies: {}
# [test] fn partial_one_of_test () { fn f < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , u8 > { one_of (['a' , 'b']) . parse_next (i) } let a = & b"abcd" [..] ; assert_parse ! (f . parse_peek (Partial :: new (a)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                98,
                99,
                100,
            ],
            partial: true,
        },
        97,
    ),
)

"#]] . raw ()) ; let b = & b"cde" [..] ; assert_parse ! (f . parse_peek (Partial :: new (b)) , str ! [[r#"
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

"#]] . raw ()) ; fn utf8 < 'i > (i : & mut Partial < & 'i str >) -> TestResult < Partial < & 'i str > , char > { one_of (['+' , '\u{FF0B}']) . parse_next (i) } assert ! (utf8 . parse_peek (Partial :: new ("+")) . is_ok ()) ; assert ! (utf8 . parse_peek (Partial :: new ("\u{FF0B}")) . is_ok ()) ; }
};
}
