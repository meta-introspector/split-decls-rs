// Generated macro for complete_take_until_empty (function)
macro_rules! Depcrate_token_testscomplete_take_until_empty {
() => {
// Module: crate::token::tests
// Provides: {"complete_take_until_empty"}
// Dependencies: {}
# [test] fn complete_take_until_empty () { fn take_until_empty < 'i > (i : & mut & 'i str) -> TestResult < & 'i str , & 'i str > { take_until (0 , "") . parse_next (i) } assert_parse ! (take_until_empty . parse_peek ("") , str ! [[r#"
Ok(
    (
        "",
        "",
    ),
)

"#]] . raw ()) ; assert_parse ! (take_until_empty . parse_peek ("end") , str ! [[r#"
Ok(
    (
        "end",
        "",
    ),
)

"#]] . raw ()) ; }
};
}
