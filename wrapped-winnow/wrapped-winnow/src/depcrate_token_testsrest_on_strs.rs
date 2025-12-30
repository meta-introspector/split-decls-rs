// Generated macro for rest_on_strs (function)
macro_rules! Depcrate_token_testsrest_on_strs {
() => {
// Module: crate::token::tests
// Provides: {"rest_on_strs"}
// Dependencies: {}
# [test] fn rest_on_strs () { let input : & str = "Hello, world!" ; assert_parse ! (rest . parse_peek (input) , str ! [[r#"
Ok(
    (
        "",
        "Hello, world!",
    ),
)

"#]] . raw ()) ; }
};
}
