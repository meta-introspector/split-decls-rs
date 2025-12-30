// Generated macro for complete (function)
macro_rules! Depcrate_combinator_testscomplete {
() => {
// Module: crate::combinator::tests
// Provides: {"complete"}
// Dependencies: {}
# [test] fn complete () { fn err_test < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , & 'i [u8] > { let _ = "ijkl" . parse_next (i) ? ; "mnop" . parse_next (i) } let a = & b"ijklmn" [..] ; let res_a = err_test . parse_peek (a) ; assert_parse ! (res_a , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: [
                109,
                110,
            ],
        },
    ),
)

"#]] . raw ()) ; }
};
}
