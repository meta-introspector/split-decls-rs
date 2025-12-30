// Generated macro for test_parser_flat_map (function)
macro_rules! Depcrate_combinator_teststest_parser_flat_map {
() => {
// Module: crate::combinator::tests
// Provides: {"test_parser_flat_map"}
// Dependencies: {}
# [test] fn test_parser_flat_map () { let input : & [u8] = & [3 , 100 , 101 , 102 , 103 , 104] [..] ; assert_parse ! (u8 . flat_map (take) . parse_peek (input) , str ! [[r#"
Ok(
    (
        [
            103,
            104,
        ],
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
