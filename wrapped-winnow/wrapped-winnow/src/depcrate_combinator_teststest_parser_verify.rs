// Generated macro for test_parser_verify (function)
macro_rules! Depcrate_combinator_teststest_parser_verify {
() => {
// Module: crate::combinator::tests
// Provides: {"test_parser_verify"}
// Dependencies: {}
# [test] fn test_parser_verify () { use crate :: token :: take ; fn test < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { take (5u8) . verify (| slice : & [u8] | slice [0] == b'a') . parse_next (i) } assert_parse ! (test . parse_peek (Partial :: new (& b"bcd" [..])) , str ! [[r#"
Err(
    Incomplete(
        Size(
            2,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (test . parse_peek (Partial :: new (& b"bcdefg" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    98,
                    99,
                    100,
                    101,
                    102,
                    103,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (test . parse_peek (Partial :: new (& b"abcdefg" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                102,
                103,
            ],
            partial: true,
        },
        [
            97,
            98,
            99,
            100,
            101,
        ],
    ),
)

"#]] . raw ()) ; }
};
}
