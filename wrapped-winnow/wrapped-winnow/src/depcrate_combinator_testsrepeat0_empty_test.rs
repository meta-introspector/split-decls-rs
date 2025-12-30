// Generated macro for repeat0_empty_test (function)
macro_rules! Depcrate_combinator_testsrepeat0_empty_test {
() => {
// Module: crate::combinator::tests
// Provides: {"repeat0_empty_test"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] # [cfg_attr (debug_assertions , should_panic)] fn repeat0_empty_test () { fn multi_empty < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , Vec < & 'i [u8] > > { repeat (0 .. , "") . parse_next (i) } assert_parse ! (multi_empty . parse_peek (Partial :: new (& b"abcdef" [..])) , str ! [[r#"
Err(
    Cut(
        InputError {
            input: Partial {
                input: [
                    97,
                    98,
                    99,
                    100,
                    101,
                    102,
                ],
                partial: true,
            },
        },
    ),
)

"#]]) ; }
};
}
