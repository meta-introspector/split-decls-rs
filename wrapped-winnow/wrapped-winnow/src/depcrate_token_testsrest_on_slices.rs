// Generated macro for rest_on_slices (function)
macro_rules! Depcrate_token_testsrest_on_slices {
() => {
// Module: crate::token::tests
// Provides: {"rest_on_slices"}
// Dependencies: {}
# [test] fn rest_on_slices () { let input : & [u8] = & b"Hello, world!" [..] ; assert_parse ! (rest . parse_peek (input) , str ! [[r#"
Ok(
    (
        [],
        [
            72,
            101,
            108,
            108,
            111,
            44,
            32,
            119,
            111,
            114,
            108,
            100,
            33,
        ],
    ),
)

"#]] . raw ()) ; }
};
}
