// Generated macro for repeat_till_range_test (function)
macro_rules! Depcrate_combinator_testsrepeat_till_range_test {
() => {
// Module: crate::combinator::tests
// Provides: {"repeat_till_range_test"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn repeat_till_range_test () { # [allow (clippy :: type_complexity)] fn multi < 'i > (i : & mut & 'i str) -> TestResult < & 'i str , (Vec < & 'i str > , & 'i str) > { repeat_till (2 ..= 4 , "ab" , "cd") . parse_next (i) } assert_parse ! (multi . parse_peek ("cd") , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "cd",
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek ("abcd") , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "cd",
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek ("ababcd") , str ! [[r#"
Ok(
    (
        "",
        (
            [
                "ab",
                "ab",
            ],
            "cd",
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek ("abababcd") , str ! [[r#"
Ok(
    (
        "",
        (
            [
                "ab",
                "ab",
                "ab",
            ],
            "cd",
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek ("ababababcd") , str ! [[r#"
Ok(
    (
        "",
        (
            [
                "ab",
                "ab",
                "ab",
                "ab",
            ],
            "cd",
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (multi . parse_peek ("abababababcd") , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "abcd",
        },
    ),
)

"#]] . raw ()) ; }
};
}
