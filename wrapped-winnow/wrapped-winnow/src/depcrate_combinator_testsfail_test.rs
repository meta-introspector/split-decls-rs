// Generated macro for fail_test (function)
macro_rules! Depcrate_combinator_testsfail_test {
() => {
// Module: crate::combinator::tests
// Provides: {"fail_test"}
// Dependencies: {}
# [test] fn fail_test () { let a = "string" ; let b = "another string" ; assert_parse ! (fail ::< _ , & str , _ >. parse_peek (a) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "string",
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (fail ::< _ , & str , _ >. parse_peek (b) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "another string",
        },
    ),
)

"#]] . raw ()) ; }
};
}
