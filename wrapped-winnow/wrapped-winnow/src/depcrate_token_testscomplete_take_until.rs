// Generated macro for complete_take_until (function)
macro_rules! Depcrate_token_testscomplete_take_until {
() => {
// Module: crate::token::tests
// Provides: {"complete_take_until"}
// Dependencies: {}
# [test] fn complete_take_until () { fn take_until_5_10 < 'i > (i : & mut & 'i str) -> TestResult < & 'i str , & 'i str > { take_until (5 ..= 8 , "end") . parse_next (i) } assert_parse ! (take_until_5_10 . parse_peek ("end") , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "end",
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (take_until_5_10 . parse_peek ("1234end") , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "1234end",
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (take_until_5_10 . parse_peek ("12345end") , str ! [[r#"
Ok(
    (
        "end",
        "12345",
    ),
)

"#]] . raw ()) ; assert_parse ! (take_until_5_10 . parse_peek ("123456end") , str ! [[r#"
Ok(
    (
        "end",
        "123456",
    ),
)

"#]] . raw ()) ; assert_parse ! (take_until_5_10 . parse_peek ("12345678end") , str ! [[r#"
Ok(
    (
        "end",
        "12345678",
    ),
)

"#]] . raw ()) ; assert_parse ! (take_until_5_10 . parse_peek ("123456789end") , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "123456789end",
        },
    ),
)

"#]] . raw ()) ; }
};
}
