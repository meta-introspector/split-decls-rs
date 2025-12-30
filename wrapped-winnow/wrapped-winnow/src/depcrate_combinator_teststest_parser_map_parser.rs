// Generated macro for test_parser_map_parser (function)
macro_rules! Depcrate_combinator_teststest_parser_map_parser {
() => {
// Module: crate::combinator::tests
// Provides: {"test_parser_map_parser"}
// Dependencies: {}
# [test] fn test_parser_map_parser () { let input : & [u8] = & [100 , 101 , 102 , 103 , 104] [..] ; assert_parse ! (take (4usize) . and_then (take (2usize)) . parse_peek (input) , str ! [[r#"
Ok(
    (
        [
            104,
        ],
        [
            100,
            101,
        ],
    ),
)

"#]] . raw ()) ; }
};
}
