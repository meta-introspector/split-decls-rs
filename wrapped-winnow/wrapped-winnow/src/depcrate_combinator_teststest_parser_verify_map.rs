// Generated macro for test_parser_verify_map (function)
macro_rules! Depcrate_combinator_teststest_parser_verify_map {
() => {
// Module: crate::combinator::tests
// Provides: {"test_parser_verify_map"}
// Dependencies: {}
# [test] fn test_parser_verify_map () { let input : & [u8] = & [50] [..] ; assert_parse ! (u8 . verify_map (| u | if u < 20 { Some (u) } else { None }) . parse_peek (input) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: [
                50,
            ],
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (u8 . verify_map (| u | if u > 20 { Some (u) } else { None }) . parse_peek (input) , str ! [[r#"
Ok(
    (
        [],
        50,
    ),
)

"#]] . raw ()) ; }
};
}
