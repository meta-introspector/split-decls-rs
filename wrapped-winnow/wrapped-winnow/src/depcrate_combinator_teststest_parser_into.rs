// Generated macro for test_parser_into (function)
macro_rules! Depcrate_combinator_teststest_parser_into {
() => {
// Module: crate::combinator::tests
// Provides: {"test_parser_into"}
// Dependencies: {}
# [test] # [cfg (feature = "std")] fn test_parser_into () { use crate :: token :: take ; assert_parse ! (take (3u8) . output_into ::< Vec < u8 >> () . parse_peek (& b"abcdefg" [..]) , str ! [[r#"
Ok(
    (
        [
            100,
            101,
            102,
            103,
        ],
        [
            97,
            98,
            99,
        ],
    ),
)

"#]] . raw ()) ; }
};
}
