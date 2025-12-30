// Generated macro for complete_literal_fixed_size_array (function)
macro_rules! Depcrate_token_testscomplete_literal_fixed_size_array {
() => {
// Module: crate::token::tests
// Provides: {"complete_literal_fixed_size_array"}
// Dependencies: {}
# [test] fn complete_literal_fixed_size_array () { fn test < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , & 'i [u8] > { literal ([0x42]) . parse_next (i) } fn test2 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , & 'i [u8] > { literal (& [0x42]) . parse_next (i) } let input = & [0x42 , 0x00] [..] ; assert_parse ! (test . parse_peek (input) , str ! [[r#"
Ok(
    (
        [
            0,
        ],
        [
            66,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (test2 . parse_peek (input) , str ! [[r#"
Ok(
    (
        [
            0,
        ],
        [
            66,
        ],
    ),
)

"#]] . raw ()) ; }
};
}
