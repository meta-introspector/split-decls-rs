// Generated macro for repeat1_count_test (function)
macro_rules! Depcrate_combinator_testsrepeat1_count_test {
() => {
// Module: crate::combinator::tests
// Provides: {"repeat1_count_test"}
// Dependencies: {}
# [test] fn repeat1_count_test () { fn count1_nums < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , usize > { repeat (1 .. , (digit , ",")) . parse_next (i) } assert_parse ! (count1_nums . parse_peek (& b"123,45,junk" [..]) , str ! [[r#"
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

"#]] . raw ()) ; assert_parse ! (count1_nums . parse_peek (& b"1,2,3,4,5,6,7,8,9,0,junk" [..]) , str ! [[r#"
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

"#]] . raw ()) ; assert_parse ! (count1_nums . parse_peek (& b"hello" [..]) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: [
                104,
                101,
                108,
                108,
                111,
            ],
        },
    ),
)

"#]] . raw ()) ; }
};
}
