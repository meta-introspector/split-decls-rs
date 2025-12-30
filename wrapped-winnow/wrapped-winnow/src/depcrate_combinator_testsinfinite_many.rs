// Generated macro for infinite_many (function)
macro_rules! Depcrate_combinator_testsinfinite_many {
() => {
// Module: crate::combinator::tests
// Provides: {"infinite_many"}
// Dependencies: {}
# [test] # [cfg (feature = "std")] fn infinite_many () { fn tst < 'i > (input : & mut & 'i [u8]) -> TestResult < & 'i [u8] , & 'i [u8] > { println ! ("input: {input:?}") ; Err (ParserError :: from_input (input)) } fn multi0 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , Vec < & 'i [u8] > > { repeat (0 .. , tst) . parse_next (i) } let a = & b"abcdef" [..] ; assert_parse ! (multi0 . parse_peek (a) , str ! [[r#"
Ok(
    (
        [
            97,
            98,
            99,
            100,
            101,
            102,
        ],
        [],
    ),
)

"#]] . raw ()) ; fn multi1 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , Vec < & 'i [u8] > > { repeat (1 .. , tst) . parse_next (i) } let a = & b"abcdef" [..] ; assert_parse ! (multi1 . parse_peek (a) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: [
                97,
                98,
                99,
                100,
                101,
                102,
            ],
        },
    ),
)

"#]] . raw ()) ; }
};
}
