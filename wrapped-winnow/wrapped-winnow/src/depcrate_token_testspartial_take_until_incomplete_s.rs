// Generated macro for partial_take_until_incomplete_s (function)
macro_rules! Depcrate_token_testspartial_take_until_incomplete_s {
() => {
// Module: crate::token::tests
// Provides: {"partial_take_until_incomplete_s"}
// Dependencies: {}
# [test] fn partial_take_until_incomplete_s () { fn ys < 'i > (i : & mut Partial < & 'i str >) -> TestResult < Partial < & 'i str > , & 'i str > { take_until (0 .. , "end") . parse_next (i) } assert_parse ! (ys . parse_peek (Partial :: new ("123en")) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; }
};
}
