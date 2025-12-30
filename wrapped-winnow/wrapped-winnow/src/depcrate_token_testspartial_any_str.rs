// Generated macro for partial_any_str (function)
macro_rules! Depcrate_token_testspartial_any_str {
() => {
// Module: crate::token::tests
// Provides: {"partial_any_str"}
// Dependencies: {}
# [test] fn partial_any_str () { use super :: any ; assert_parse ! (any . parse_peek (Partial :: new ("Ә")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "",
            partial: true,
        },
        'Ә',
    ),
)

"#]] . raw ()) ; }
};
}
