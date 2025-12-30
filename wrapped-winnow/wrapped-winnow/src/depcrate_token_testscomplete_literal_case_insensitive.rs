// Generated macro for complete_literal_case_insensitive (function)
macro_rules! Depcrate_token_testscomplete_literal_case_insensitive {
() => {
// Module: crate::token::tests
// Provides: {"complete_literal_case_insensitive"}
// Dependencies: {}
# [test] fn complete_literal_case_insensitive () { fn caseless_bytes < 'i > (i : & mut & 'i [u8]) -> TestResult < & 'i [u8] , & 'i [u8] > { literal (Caseless ("ABcd")) . parse_next (i) } assert_parse ! (caseless_bytes . parse_peek (& b"aBCdefgh" [..]) , str ! [[r#"
Ok(
    (
        [
            101,
            102,
            103,
            104,
        ],
        [
            97,
            66,
            67,
            100,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_bytes . parse_peek (& b"abcdefgh" [..]) , str ! [[r#"
Ok(
    (
        [
            101,
            102,
            103,
            104,
        ],
        [
            97,
            98,
            99,
            100,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_bytes . parse_peek (& b"ABCDefgh" [..]) , str ! [[r#"
Ok(
    (
        [
            101,
            102,
            103,
            104,
        ],
        [
            65,
            66,
            67,
            68,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_bytes . parse_peek (& b"ab" [..]) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: [
                97,
                98,
            ],
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_bytes . parse_peek (& b"Hello" [..]) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: [
                72,
                101,
                108,
                108,
                111,
            ],
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_bytes . parse_peek (& b"Hel" [..]) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: [
                72,
                101,
                108,
            ],
        },
    ),
)

"#]] . raw ()) ; fn caseless_str < 'i > (i : & mut & 'i str) -> TestResult < & 'i str , & 'i str > { literal (Caseless ("ABcd")) . parse_next (i) } assert_parse ! (caseless_str . parse_peek ("aBCdefgh") , str ! [[r#"
Ok(
    (
        "efgh",
        "aBCd",
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_str . parse_peek ("abcdefgh") , str ! [[r#"
Ok(
    (
        "efgh",
        "abcd",
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_str . parse_peek ("ABCDefgh") , str ! [[r#"
Ok(
    (
        "efgh",
        "ABCD",
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_str . parse_peek ("ab") , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "ab",
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_str . parse_peek ("Hello") , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "Hello",
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (caseless_str . parse_peek ("Hel") , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "Hel",
        },
    ),
)

"#]] . raw ()) ; fn matches_kelvin < 'i > (i : & mut & 'i str) -> TestResult < & 'i str , & 'i str > { literal (Caseless ("k")) . parse_next (i) } assert_parse ! (matches_kelvin . parse_peek ("K") , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "K",
        },
    ),
)

"#]] . raw ()) ; fn is_kelvin < 'i > (i : & mut & 'i str) -> TestResult < & 'i str , & 'i str > { literal (Caseless ("K")) . parse_next (i) } assert_parse ! (is_kelvin . parse_peek ("k") , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: "k",
        },
    ),
)

"#]] . raw ()) ; }
};
}
