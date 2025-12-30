// Generated macro for complete_literal_char (function)
macro_rules! Depcrate_token_testscomplete_literal_char {
() => {
// Module: crate::token::tests
// Provides: {"complete_literal_char"}
// Dependencies: {}
# [test] fn complete_literal_char () { fn test < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , & 'i [u8] > { literal ('B') . parse_next (i) } assert_parse ! (test . parse_peek (& [0x42 , 0x00] [..]) , str ! [[r#"
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

"#]] . raw ()) ; assert_parse ! (test . parse_peek (& [b'A' , b'\0'] [..]) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: [
                65,
                0,
            ],
        },
    ),
)

"#]] . raw ()) ; }
};
}
