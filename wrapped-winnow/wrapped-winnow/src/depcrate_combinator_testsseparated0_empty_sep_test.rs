// Generated macro for separated0_empty_sep_test (function)
macro_rules! Depcrate_combinator_testsseparated0_empty_sep_test {
() => {
// Module: crate::combinator::tests
// Provides: {"separated0_empty_sep_test"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] # [cfg_attr (debug_assertions , should_panic)] fn separated0_empty_sep_test () { fn empty_sep < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , Vec < & 'i [u8] > > { separated (0 .. , "abc" , "") . parse_next (i) } let i = & b"abcabc" [..] ; assert_parse ! (empty_sep . parse_peek (Partial :: new (i)) , str ! [[r#"
Err(
    Cut(
        InputError {
            input: Partial {
                input: [
                    97,
                    98,
                    99,
                ],
                partial: true,
            },
        },
    ),
)

"#]]) ; }
};
}
