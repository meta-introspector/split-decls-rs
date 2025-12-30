// Generated macro for partial_take_take_while0 (function)
macro_rules! Depcrate_token_testspartial_take_take_while0 {
() => {
// Module: crate::token::tests
// Provides: {"partial_take_take_while0"}
// Dependencies: {}
# [test] # [cfg (feature = "std")] fn partial_take_take_while0 () { fn x < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { take_while (0 .. , AsChar :: is_alphanum) . parse_next (i) } fn y < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { x . take () . parse_next (i) } assert_parse ! (x . parse_peek (Partial :: new (& b"ab." [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                46,
            ],
            partial: true,
        },
        [
            97,
            98,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (y . parse_peek (Partial :: new (& b"ab." [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                46,
            ],
            partial: true,
        },
        [
            97,
            98,
        ],
    ),
)

"#]] . raw ()) ; }
};
}
