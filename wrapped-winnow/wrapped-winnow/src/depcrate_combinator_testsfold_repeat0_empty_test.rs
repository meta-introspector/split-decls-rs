// Generated macro for fold_repeat0_empty_test (function)
macro_rules! Depcrate_combinator_testsfold_repeat0_empty_test {
() => {
// Module: crate::combinator::tests
// Provides: {"fold_repeat0_empty_test"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] # [cfg_attr (debug_assertions , should_panic)] fn fold_repeat0_empty_test () { fn fold_into_vec < T > (mut acc : Vec < T > , item : T) -> Vec < T > { acc . push (item) ; acc } fn multi_empty < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , Vec < & 'i [u8] > > { repeat (0 .. , "") . fold (Vec :: new , fold_into_vec) . parse_next (i) } assert_parse ! (multi_empty . parse_peek (Partial :: new (& b"abcdef" [..])) , str ! [[r#"
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
