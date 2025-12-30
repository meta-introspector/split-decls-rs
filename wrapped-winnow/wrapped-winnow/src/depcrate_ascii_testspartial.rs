// Generated macro for partial (module)
macro_rules! Depcrate_ascii_testspartial {
() => {
// Module: crate::ascii::tests
// Provides: {"partial"}
// Dependencies: {}
mod partial { use super :: * ; use crate :: error :: InputError ; use crate :: prelude :: * ; use crate :: Partial ; # [test] fn character () { let a : & [u8] = b"abcd" ; let b : & [u8] = b"1234" ; let c : & [u8] = b"a123" ; let d : & [u8] = "azé12" . as_bytes () ; let e : & [u8] = b" " ; let f : & [u8] = b" ;" ; assert_parse ! (alpha1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (alpha1 . parse_peek (Partial :: new (b)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    49,
                    50,
                    51,
                    52,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (alpha1 . parse_peek (Partial :: new (c)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                49,
                50,
                51,
            ],
            partial: true,
        },
        [
            97,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (alpha1 . parse_peek (Partial :: new (d)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                195,
                169,
                49,
                50,
            ],
            partial: true,
        },
        [
            97,
            122,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (digit1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    97,
                    98,
                    99,
                    100,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (digit1 . parse_peek (Partial :: new (b)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (digit1 . parse_peek (Partial :: new (c)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    97,
                    49,
                    50,
                    51,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (digit1 . parse_peek (Partial :: new (d)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    97,
                    122,
                    195,
                    169,
                    49,
                    50,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_digit1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_digit1 . parse_peek (Partial :: new (b)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_digit1 . parse_peek (Partial :: new (c)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_digit1 . parse_peek (Partial :: new (d)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                122,
                195,
                169,
                49,
                50,
            ],
            partial: true,
        },
        [
            97,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_digit1 . parse_peek (Partial :: new (e)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    32,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (oct_digit1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    97,
                    98,
                    99,
                    100,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (oct_digit1 . parse_peek (Partial :: new (b)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (oct_digit1 . parse_peek (Partial :: new (c)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    97,
                    49,
                    50,
                    51,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (oct_digit1 . parse_peek (Partial :: new (d)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    97,
                    122,
                    195,
                    169,
                    49,
                    50,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (alphanumeric1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (alphanumeric1 . parse_peek (Partial :: new (c)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (alphanumeric1 . parse_peek (Partial :: new (d)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                195,
                169,
                49,
                50,
            ],
            partial: true,
        },
        [
            97,
            122,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (space1 . parse_peek (Partial :: new (e)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (space1 . parse_peek (Partial :: new (f)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                59,
            ],
            partial: true,
        },
        [
            32,
        ],
    ),
)

"#]] . raw ()) ; } # [cfg (feature = "alloc")] # [test] fn character_s () { let a = "abcd" ; let b = "1234" ; let c = "a123" ; let d = "azé12" ; let e = " " ; assert_parse ! (alpha1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (alpha1 . parse_peek (Partial :: new (b)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: "1234",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (alpha1 . parse_peek (Partial :: new (c)) , str ! [[r#"
Ok(
    (
        Partial {
            input: "123",
            partial: true,
        },
        "a",
    ),
)

"#]] . raw ()) ; assert_parse ! (alpha1 . parse_peek (Partial :: new (d)) , str ! [[r#"
Ok(
    (
        Partial {
            input: "é12",
            partial: true,
        },
        "az",
    ),
)

"#]] . raw ()) ; assert_parse ! (digit1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: "abcd",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (digit1 . parse_peek (Partial :: new (b)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (digit1 . parse_peek (Partial :: new (c)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: "a123",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (digit1 . parse_peek (Partial :: new (d)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: "azé12",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_digit1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_digit1 . parse_peek (Partial :: new (b)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_digit1 . parse_peek (Partial :: new (c)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_digit1 . parse_peek (Partial :: new (d)) , str ! [[r#"
Ok(
    (
        Partial {
            input: "zé12",
            partial: true,
        },
        "a",
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_digit1 . parse_peek (Partial :: new (e)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: " ",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (oct_digit1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: "abcd",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (oct_digit1 . parse_peek (Partial :: new (b)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (oct_digit1 . parse_peek (Partial :: new (c)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: "a123",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (oct_digit1 . parse_peek (Partial :: new (d)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: "azé12",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (alphanumeric1 . parse_peek (Partial :: new (a)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (alphanumeric1 . parse_peek (Partial :: new (c)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; assert_parse ! (alphanumeric1 . parse_peek (Partial :: new (d)) , str ! [[r#"
Ok(
    (
        Partial {
            input: "é12",
            partial: true,
        },
        "az",
    ),
)

"#]] . raw ()) ; assert_parse ! (space1 . parse_peek (Partial :: new (e)) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; } use crate :: stream :: Offset ; # [test] fn offset () { let a = & b"abcd;" [..] ; let b = & b"1234;" [..] ; let c = & b"a123;" [..] ; let d = & b" \t;" [..] ; let e = & b" \t\r\n;" [..] ; let f = & b"123abcDEF;" [..] ; match alpha1 :: < _ , InputError < _ > > . parse_peek (Partial :: new (a)) { Ok ((i , _)) => { let i = i . into_inner () ; assert_eq ! (i . offset_from (& a) + i . len () , a . len ()) ; } _ => panic ! ("wrong return type in offset test for alpha") , } match digit1 :: < _ , InputError < _ > > . parse_peek (Partial :: new (b)) { Ok ((i , _)) => { let i = i . into_inner () ; assert_eq ! (i . offset_from (& b) + i . len () , b . len ()) ; } _ => panic ! ("wrong return type in offset test for digit") , } match alphanumeric1 :: < _ , InputError < _ > > . parse_peek (Partial :: new (c)) { Ok ((i , _)) => { let i = i . into_inner () ; assert_eq ! (i . offset_from (& c) + i . len () , c . len ()) ; } _ => panic ! ("wrong return type in offset test for alphanumeric") , } match space1 :: < _ , InputError < _ > > . parse_peek (Partial :: new (d)) { Ok ((i , _)) => { let i = i . into_inner () ; assert_eq ! (i . offset_from (& d) + i . len () , d . len ()) ; } _ => panic ! ("wrong return type in offset test for space") , } match multispace1 :: < _ , InputError < _ > > . parse_peek (Partial :: new (e)) { Ok ((i , _)) => { let i = i . into_inner () ; assert_eq ! (i . offset_from (& e) + i . len () , e . len ()) ; } _ => panic ! ("wrong return type in offset test for multispace") , } match hex_digit1 :: < _ , InputError < _ > > . parse_peek (Partial :: new (f)) { Ok ((i , _)) => { let i = i . into_inner () ; assert_eq ! (i . offset_from (& f) + i . len () , f . len ()) ; } _ => panic ! ("wrong return type in offset test for hex_digit") , } match oct_digit1 :: < _ , InputError < _ > > . parse_peek (Partial :: new (f)) { Ok ((i , _)) => { let i = i . into_inner () ; assert_eq ! (i . offset_from (& f) + i . len () , f . len ()) ; } _ => panic ! ("wrong return type in offset test for oct_digit") , } } # [test] fn is_till_line_ending_bytes () { let a : & [u8] = b"ab12cd\nefgh" ; assert_parse ! (till_line_ending . parse_peek (Partial :: new (a)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                10,
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
            49,
            50,
            99,
            100,
        ],
    ),
)

"#]] . raw ()) ; let b : & [u8] = b"ab12cd\nefgh\nijkl" ; assert_parse ! (till_line_ending . parse_peek (Partial :: new (b)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                10,
                101,
                102,
                103,
                104,
                10,
                105,
                106,
                107,
                108,
            ],
            partial: true,
        },
        [
            97,
            98,
            49,
            50,
            99,
            100,
        ],
    ),
)

"#]] . raw ()) ; let c : & [u8] = b"ab12cd\r\nefgh\nijkl" ; assert_parse ! (till_line_ending . parse_peek (Partial :: new (c)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                13,
                10,
                101,
                102,
                103,
                104,
                10,
                105,
                106,
                107,
                108,
            ],
            partial: true,
        },
        [
            97,
            98,
            49,
            50,
            99,
            100,
        ],
    ),
)

"#]] . raw ()) ; let d : & [u8] = b"ab12cd" ; assert_parse ! (till_line_ending . parse_peek (Partial :: new (d)) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; } # [test] fn is_till_line_ending_str () { let f = "βèƒôřè\rÂßÇáƒƭèř" ; assert_parse ! (till_line_ending . parse_peek (Partial :: new (f)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: "\rÂßÇáƒƭèř",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; let g2 : & str = "ab12cd" ; assert_parse ! (till_line_ending . parse_peek (Partial :: new (g2)) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; } # [test] fn hex_digit_test () { let i = & b"0123456789abcdefABCDEF;" [..] ; assert_parse ! (hex_digit1 . parse_peek (Partial :: new (i)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                59,
            ],
            partial: true,
        },
        [
            48,
            49,
            50,
            51,
            52,
            53,
            54,
            55,
            56,
            57,
            97,
            98,
            99,
            100,
            101,
            102,
            65,
            66,
            67,
            68,
            69,
            70,
        ],
    ),
)

"#]] . raw ()) ; let i = & b"g" [..] ; assert_parse ! (hex_digit1 . parse_peek (Partial :: new (i)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    103,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; let i = & b"G" [..] ; assert_parse ! (hex_digit1 . parse_peek (Partial :: new (i)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    71,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert ! (AsChar :: is_hex_digit (b'0')) ; assert ! (AsChar :: is_hex_digit (b'9')) ; assert ! (AsChar :: is_hex_digit (b'a')) ; assert ! (AsChar :: is_hex_digit (b'f')) ; assert ! (AsChar :: is_hex_digit (b'A')) ; assert ! (AsChar :: is_hex_digit (b'F')) ; assert ! (! AsChar :: is_hex_digit (b'g')) ; assert ! (! AsChar :: is_hex_digit (b'G')) ; assert ! (! AsChar :: is_hex_digit (b'/')) ; assert ! (! AsChar :: is_hex_digit (b':')) ; assert ! (! AsChar :: is_hex_digit (b'@')) ; assert ! (! AsChar :: is_hex_digit (b'\x60')) ; } # [test] fn oct_digit_test () { let i = & b"01234567;" [..] ; assert_parse ! (oct_digit1 . parse_peek (Partial :: new (i)) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                59,
            ],
            partial: true,
        },
        [
            48,
            49,
            50,
            51,
            52,
            53,
            54,
            55,
        ],
    ),
)

"#]] . raw ()) ; let i = & b"8" [..] ; assert_parse ! (oct_digit1 . parse_peek (Partial :: new (i)) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    56,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert ! (AsChar :: is_oct_digit (b'0')) ; assert ! (AsChar :: is_oct_digit (b'7')) ; assert ! (! AsChar :: is_oct_digit (b'8')) ; assert ! (! AsChar :: is_oct_digit (b'9')) ; assert ! (! AsChar :: is_oct_digit (b'a')) ; assert ! (! AsChar :: is_oct_digit (b'A')) ; assert ! (! AsChar :: is_oct_digit (b'/')) ; assert ! (! AsChar :: is_oct_digit (b':')) ; assert ! (! AsChar :: is_oct_digit (b'@')) ; assert ! (! AsChar :: is_oct_digit (b'\x60')) ; } # [test] fn full_line_windows () { # [allow (clippy :: type_complexity)] fn take_full_line < 'i > (i : & mut Partial < & 'i [u8] > ,) -> TestResult < Partial < & 'i [u8] > , (& 'i [u8] , & 'i [u8]) > { (till_line_ending , line_ending) . parse_next (i) } let input = b"abc\r\n" ; let output = take_full_line . parse_peek (Partial :: new (input)) ; assert_parse ! (output , str ! [[r#"
Ok(
    (
        Partial {
            input: [],
            partial: true,
        },
        (
            [
                97,
                98,
                99,
            ],
            [
                13,
                10,
            ],
        ),
    ),
)

"#]] . raw ()) ; } # [test] fn full_line_unix () { # [allow (clippy :: type_complexity)] fn take_full_line < 'i > (i : & mut Partial < & 'i [u8] > ,) -> TestResult < Partial < & 'i [u8] > , (& 'i [u8] , & 'i [u8]) > { (till_line_ending , line_ending) . parse_next (i) } let input = b"abc\n" ; let output = take_full_line . parse_peek (Partial :: new (input)) ; assert_parse ! (output , str ! [[r#"
Ok(
    (
        Partial {
            input: [],
            partial: true,
        },
        (
            [
                97,
                98,
                99,
            ],
            [
                10,
            ],
        ),
    ),
)

"#]] . raw ()) ; } # [test] fn check_windows_lineending () { let input = b"\r\n" ; let output = line_ending . parse_peek (Partial :: new (& input [..])) ; assert_parse ! (output , str ! [[r#"
Ok(
    (
        Partial {
            input: [],
            partial: true,
        },
        [
            13,
            10,
        ],
    ),
)

"#]] . raw ()) ; } # [test] fn check_unix_lineending () { let input = b"\n" ; let output = line_ending . parse_peek (Partial :: new (& input [..])) ; assert_parse ! (output , str ! [[r#"
Ok(
    (
        Partial {
            input: [],
            partial: true,
        },
        [
            10,
        ],
    ),
)

"#]] . raw ()) ; } # [test] fn cr_lf () { assert_parse ! (crlf . parse_peek (Partial :: new (& b"\r\na" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                97,
            ],
            partial: true,
        },
        [
            13,
            10,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (crlf . parse_peek (Partial :: new (& b"\r" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (crlf . parse_peek (Partial :: new (& b"\ra" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    13,
                    97,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (crlf . parse_peek (Partial :: new ("\r\na")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "a",
            partial: true,
        },
        "\r\n",
    ),
)

"#]] . raw ()) ; assert_parse ! (crlf . parse_peek (Partial :: new ("\r")) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (crlf . parse_peek (Partial :: new ("\ra")) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: "\ra",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; } # [test] fn end_of_line () { assert_parse ! (line_ending . parse_peek (Partial :: new (& b"\na" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                97,
            ],
            partial: true,
        },
        [
            10,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (line_ending . parse_peek (Partial :: new (& b"\r\na" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                97,
            ],
            partial: true,
        },
        [
            13,
            10,
        ],
    ),
)

"#]] . raw ()) ; assert_parse ! (line_ending . parse_peek (Partial :: new (& b"\r" [..])) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (line_ending . parse_peek (Partial :: new (& b"\ra" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    13,
                    97,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (line_ending . parse_peek (Partial :: new ("\na")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "a",
            partial: true,
        },
        "\n",
    ),
)

"#]] . raw ()) ; assert_parse ! (line_ending . parse_peek (Partial :: new ("\r\na")) , str ! [[r#"
Ok(
    (
        Partial {
            input: "a",
            partial: true,
        },
        "\r\n",
    ),
)

"#]] . raw ()) ; assert_parse ! (line_ending . parse_peek (Partial :: new ("\r")) , str ! [[r#"
Err(
    Incomplete(
        Unknown,
    ),
)

"#]] . raw ()) ; assert_parse ! (line_ending . parse_peek (Partial :: new ("\ra")) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: "\ra",
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; } # [test] fn hex_uint_tests () { fn hex_u32 < 'i > (input : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , u32 > { hex_uint . parse_next (input) } assert_parse ! (hex_u32 . parse_peek (Partial :: new (& b";" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    59,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_u32 . parse_peek (Partial :: new (& b"ff;" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                59,
            ],
            partial: true,
        },
        255,
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_u32 . parse_peek (Partial :: new (& b"1be2;" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                59,
            ],
            partial: true,
        },
        7138,
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_u32 . parse_peek (Partial :: new (& b"c5a31be2;" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                59,
            ],
            partial: true,
        },
        3315801058,
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_u32 . parse_peek (Partial :: new (& b"C5A31be2;" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                59,
            ],
            partial: true,
        },
        3315801058,
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_u32 . parse_peek (Partial :: new (& b"00c5a31be2;" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    48,
                    48,
                    99,
                    53,
                    97,
                    51,
                    49,
                    98,
                    101,
                    50,
                    59,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_u32 . parse_peek (Partial :: new (& b"c5a31be201;" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    99,
                    53,
                    97,
                    51,
                    49,
                    98,
                    101,
                    50,
                    48,
                    49,
                    59,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_u32 . parse_peek (Partial :: new (& b"ffffffff;" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                59,
            ],
            partial: true,
        },
        4294967295,
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_u32 . parse_peek (Partial :: new (& b"ffffffffffffffff;" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    59,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_u32 . parse_peek (Partial :: new (& b"ffffffffffffffff" [..])) , str ! [[r#"
Err(
    Backtrack(
        InputError {
            input: Partial {
                input: [
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                    102,
                ],
                partial: true,
            },
        },
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_u32 . parse_peek (Partial :: new (& b"0x1be2;" [..])) , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                120,
                49,
                98,
                101,
                50,
                59,
            ],
            partial: true,
        },
        0,
    ),
)

"#]] . raw ()) ; assert_parse ! (hex_u32 . parse_peek (Partial :: new (& b"12af" [..])) , str ! [[r#"
Err(
    Incomplete(
        Size(
            1,
        ),
    ),
)

"#]] . raw ()) ; } }
};
}
