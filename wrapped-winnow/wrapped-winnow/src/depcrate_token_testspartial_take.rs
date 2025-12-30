// Generated macro for partial_take (function)
macro_rules! Depcrate_token_testspartial_take {
() => {
// Module: crate::token::tests
// Provides: {"partial_take"}
// Dependencies: {}
# [test] fn partial_take () { use crate :: ascii :: { alpha1 as alpha , alphanumeric1 as alphanumeric , digit1 as digit , hex_digit1 as hex_digit , multispace1 as multispace , oct_digit1 as oct_digit , space1 as space , } ; fn x < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { delimited ("<!--" , take (5_usize) , "-->") . take () . parse_next (i) } let r = x . parse_peek (Partial :: new (& b"<!-- abc --> aaa" [..])) ; assert_parse ! (r , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                32,
                97,
                97,
                97,
            ],
            partial: true,
        },
        [
            60,
            33,
            45,
            45,
            32,
            97,
            98,
            99,
            32,
            45,
            45,
            62,
        ],
    ),
)

"#]] . raw ()) ; fn ya < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { alpha . take () . parse_next (i) } let ra = ya . parse_peek (Partial :: new (& b"abc;" [..])) ; assert_parse ! (ra , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                59,
            ],
            partial: true,
        },
        [
            97,
            98,
            99,
        ],
    ),
)

"#]] . raw ()) ; fn yd < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { digit . take () . parse_next (i) } let rd = yd . parse_peek (Partial :: new (& b"123;" [..])) ; assert_parse ! (rd , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                59,
            ],
            partial: true,
        },
        [
            49,
            50,
            51,
        ],
    ),
)

"#]] . raw ()) ; fn yhd < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { hex_digit . take () . parse_next (i) } let rhd = yhd . parse_peek (Partial :: new (& b"123abcDEF;" [..])) ; assert_parse ! (rhd , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                59,
            ],
            partial: true,
        },
        [
            49,
            50,
            51,
            97,
            98,
            99,
            68,
            69,
            70,
        ],
    ),
)

"#]] . raw ()) ; fn yod < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { oct_digit . take () . parse_next (i) } let rod = yod . parse_peek (Partial :: new (& b"1234567;" [..])) ; assert_parse ! (rod , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                59,
            ],
            partial: true,
        },
        [
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

"#]] . raw ()) ; fn yan < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { alphanumeric . take () . parse_next (i) } let ran = yan . parse_peek (Partial :: new (& b"123abc;" [..])) ; assert_parse ! (ran , str ! [[r#"
Ok(
    (
        Partial {
            input: [
                59,
            ],
            partial: true,
        },
        [
            49,
            50,
            51,
            97,
            98,
            99,
        ],
    ),
)

"#]] . raw ()) ; fn ys < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { space . take () . parse_next (i) } let rs = ys . parse_peek (Partial :: new (& b" \t;" [..])) ; assert_parse ! (rs , str ! [[r#"
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
            9,
        ],
    ),
)

"#]] . raw ()) ; fn yms < 'i > (i : & mut Partial < & 'i [u8] >) -> TestResult < Partial < & 'i [u8] > , & 'i [u8] > { multispace . take () . parse_next (i) } let rms = yms . parse_peek (Partial :: new (& b" \t\r\n;" [..])) ; assert_parse ! (rms , str ! [[r#"
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
            9,
            13,
            10,
        ],
    ),
)

"#]] . raw ()) ; }
};
}
