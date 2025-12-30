// Generated macro for test_parser_verify_ref (function)
macro_rules! Depcrate_combinator_teststest_parser_verify_ref {
() => {
// Module: crate::combinator::tests
// Provides: {"test_parser_verify_ref"}
// Dependencies: {}
# [test] # [allow (unused)] fn test_parser_verify_ref () { use crate :: token :: take ; let mut parser1 = take (3u8) . verify (| s : & [u8] | s == & b"abc" [..]) ; assert_parse ! (parser1 . parse_peek (& b"abcd" [..]) , str ! [[r#"
Ok(
    (
        [
            100,
        ],
        [
            97,
            98,
            99,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (parser1 . parse_peek (& b"defg" [..]) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: [
                100,
                101,
                102,
                103,
            ],
        },
    ),
)

"#]] . raw ()) ; fn parser2 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , u32 > { crate :: binary :: be_u32 . verify (| val : & u32 | * val < 3) . parse_next (i) } }
};
}
