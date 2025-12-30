// Generated macro for count_zero (function)
macro_rules! Depcrate_combinator_testscount_zero {
() => {
// Module: crate::combinator::tests
// Provides: {"count_zero"}
// Dependencies: {}
# [test] # [cfg (feature = "alloc")] fn count_zero () { const TIMES : usize = 0 ; fn counter_2 < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , Vec < & 'i [u8] > > { repeat (TIMES , "abc") . parse_next (i) } let done = & b"abcabcabcdef" [..] ; let incomplete_1 = & b"ab" [..] ; let incomplete_2 = & b"abcab" [..] ; let error = & b"xxx" [..] ; let error_1 = & b"xxxabcabcdef" [..] ; let error_2 = & b"abcxxxabcdef" [..] ; assert_parse ! (counter_2 . parse_peek (done) , str ! [[r#"
Ok(
    (
        [
            97,
            98,
            99,
            97,
            98,
            99,
            97,
            98,
            99,
            100,
            101,
            102,
        ],
        [],
    ),
)

"#]] . raw ()) ; assert_parse ! (counter_2 . parse_peek (incomplete_1) , str ! [[r#"
Ok(
    (
        [
            97,
            98,
        ],
        [],
    ),
)

"#]] . raw ()) ; assert_parse ! (counter_2 . parse_peek (incomplete_2) , str ! [[r#"
Ok(
    (
        [
            97,
            98,
            99,
            97,
            98,
        ],
        [],
    ),
)

"#]] . raw ()) ; assert_parse ! (counter_2 . parse_peek (error) , str ! [[r#"
Ok(
    (
        [
            120,
            120,
            120,
        ],
        [],
    ),
)

"#]] . raw ()) ; assert_parse ! (counter_2 . parse_peek (error_1) , str ! [[r#"
Ok(
    (
        [
            120,
            120,
            120,
            97,
            98,
            99,
            97,
            98,
            99,
            100,
            101,
            102,
        ],
        [],
    ),
)

"#]] . raw ()) ; assert_parse ! (counter_2 . parse_peek (error_2) , str ! [[r#"
Ok(
    (
        [
            97,
            98,
            99,
            120,
            120,
            120,
            97,
            98,
            99,
            100,
            101,
            102,
        ],
        [],
    ),
)

"#]] . raw ()) ; }
};
}
