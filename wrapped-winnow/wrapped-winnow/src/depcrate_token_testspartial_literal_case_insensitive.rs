// Generated macro for partial_literal_case_insensitive (function)
macro_rules! Depcrate_token_testspartial_literal_case_insensitive {
() => {
// Module: crate::token::tests
// Provides: {"partial_literal_case_insensitive"}
// Dependencies: {}
# [test] fn partial_literal_case_insensitive () { fn caseless_bytes < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { literal (Caseless ("ABcd")) . parse_next (i) } assert_parse ! (caseless_bytes . parse_peek (Partial :: new (& b"aBCdefgh" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                101,
                102,
                103,
                104,
            ],
            partial: true,
        },
        [
            97,
            66,
            67,
            100,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_bytes . parse_peek (Partial :: new (& b"abcdefgh" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                101,
                102,
                103,
                104,
            ],
            partial: true,
        },
        [
            97,
            98,
            99,
            100,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_bytes . parse_peek (Partial :: new (& b"ABCDefgh" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                101,
                102,
                103,
                104,
            ],
            partial: true,
        },
        [
            65,
            66,
            67,
            68,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_bytes . parse_peek (Partial :: new (& b"ab" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_bytes . parse_peek (Partial :: new (& b"Hello" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    72,
                    101,
                    108,
                    108,
                    111,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_bytes . parse_peek (Partial :: new (& b"Hel" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    72,
                    101,
                    108,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; fn caseless_str < 'i > (i : & mut Partial < & 'i str >) -> TestResult < Partial < & 'i str > , & 'i str > { literal (Caseless ("ABcd")) . parse_next (i) } assert_parse ! (caseless_str . parse_peek (Partial :: new ("aBCdefgh")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "efgh",
            partial: true,
        },
        "aBCd",
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_str . parse_peek (Partial :: new ("abcdefgh")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "efgh",
            partial: true,
        },
        "abcd",
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_str . parse_peek (Partial :: new ("ABCDefgh")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "efgh",
            partial: true,
        },
        "ABCD",
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_str . parse_peek (Partial :: new ("ab")) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_str . parse_peek (Partial :: new ("Hello")) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: "Hello",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_str . parse_peek (Partial :: new ("Hel")) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: "Hel",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; fn matches_kelvin < 'i > (i : & mut Partial < & 'i str >) -> TestResult < Partial < & 'i str > , & 'i str > { literal (Caseless ("k")) . parse_next (i) } assert_parse ! (matches_kelvin . parse_peek (Partial :: new ("K")) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: "K",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; fn is_kelvin < 'i > (i : & mut Partial < & 'i str >) -> TestResult < Partial < & 'i str > , & 'i str > { literal (Caseless ("K")) . parse_next (i) } assert_parse ! (is_kelvin . parse_peek (Partial :: new ("k")) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: "k",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; }
};
}
