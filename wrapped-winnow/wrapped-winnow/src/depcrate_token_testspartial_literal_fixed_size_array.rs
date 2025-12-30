// Generated macro for partial_literal_fixed_size_array (function)
macro_rules! Depcrate_token_testspartial_literal_fixed_size_array {
() => {
// Module: crate::token::tests
// Provides: {"partial_literal_fixed_size_array"}
// Dependencies: {}
# [test] fn partial_literal_fixed_size_array () { fn test < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { literal ([0x42]) . parse_next (i) } fn test2 < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { literal (& [0x42]) . parse_next (i) } let input = Partial :: new (& [0x42 , 0x00] [..]) ; assert_parse ! (test . parse_peek (input) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                0,
            ],
            partial: true,
        },
        [
            66,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (test2 . parse_peek (input) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                0,
            ],
            partial: true,
        },
        [
            66,
        ],
    ),
)

"#]] . raw ()) ; }
};
}
