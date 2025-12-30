// Generated macro for repeat0_count_test (function)
macro_rules! Depcrate_combinator_testsrepeat0_count_test {
() => {
// Module: crate::combinator::tests
// Provides: {"repeat0_count_test"}
// Dependencies: {}
# [test] fn repeat0_count_test () { fn count0_nums < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , usize > { repeat (0 .. , (digit , ",")) . parse_next (i) } assert_parse ! (count0_nums . parse_peek (& b"123,junk" [..]) , str ! [[r#"
Ok(
    (
        [
            106,
            117,
            110,
            107,
        ],
        1,
    ),
)

"#]] . raw ()) ; assert_parse ! (count0_nums . parse_peek (& b"123,45,junk" [..]) , str ! [[r#"
Ok(
    (
        [
            106,
            117,
            110,
            107,
        ],
        2,
    ),
)

"#]] . raw ()) ; assert_parse ! (count0_nums . parse_peek (& b"1,2,3,4,5,6,7,8,9,0,junk" [..]) , str ! [[r#"
Ok(
    (
        [
            106,
            117,
            110,
            107,
        ],
        10,
    ),
)

"#]] . raw ()) ; assert_parse ! (count0_nums . parse_peek (& b"hello" [..]) , str ! [[r#"
Ok(
    (
        [
            104,
            101,
            108,
            108,
            111,
        ],
        0,
    ),
)

"#]] . raw ()) ; }
};
}
