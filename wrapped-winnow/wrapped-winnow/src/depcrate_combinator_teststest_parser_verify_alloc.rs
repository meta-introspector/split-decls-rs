// Generated macro for test_parser_verify_alloc (function)
macro_rules! Depcrate_combinator_teststest_parser_verify_alloc {
() => {
// Module: crate::combinator::tests
// Provides: {"test_parser_verify_alloc"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn test_parser_verify_alloc () { use crate :: token :: take ; let mut parser1 = take (3u8) . map (| s : & [u8] | s . to_vec ()) . verify (| s : & [u8] | s == & b"abc" [..]) ; assert_parse ! (parser1 . parse_peek (& b"abcd" [..]) , str ! [[r#"
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

"#]] . raw ()) ; }
};
}
