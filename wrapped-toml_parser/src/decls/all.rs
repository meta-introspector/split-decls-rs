macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! all {
    () => {
        deps!();
        # [test] fn all () { t (" a " , str ! [[r#"
[
    Token {
        kind: Whitespace,
        span: 0..1,
    },
    Token {
        kind: Atom,
        span: 1..2,
    },
    Token {
        kind: Whitespace,
        span: 2..3,
    },
    Token {
        kind: Eof,
        span: 3..3,
    },
]

"#]] . raw () ,) ; t (" a\t [[]] \t [] {} , . =\n# foo \r\n#foo \n " , str ! [[r#"
[
    Token {
        kind: Whitespace,
        span: 0..1,
    },
    Token {
        kind: Atom,
        span: 1..2,
    },
    Token {
        kind: Whitespace,
        span: 2..4,
    },
    Token {
        kind: LeftSquareBracket,
        span: 4..5,
    },
    Token {
        kind: LeftSquareBracket,
        span: 5..6,
    },
    Token {
        kind: RightSquareBracket,
        span: 6..7,
    },
    Token {
        kind: RightSquareBracket,
        span: 7..8,
    },
    Token {
        kind: Whitespace,
        span: 8..11,
    },
    Token {
        kind: LeftSquareBracket,
        span: 11..12,
    },
    Token {
        kind: RightSquareBracket,
        span: 12..13,
    },
    Token {
        kind: Whitespace,
        span: 13..14,
    },
    Token {
        kind: LeftCurlyBracket,
        span: 14..15,
    },
    Token {
        kind: RightCurlyBracket,
        span: 15..16,
    },
    Token {
        kind: Whitespace,
        span: 16..17,
    },
    Token {
        kind: Comma,
        span: 17..18,
    },
    Token {
        kind: Whitespace,
        span: 18..19,
    },
    Token {
        kind: Dot,
        span: 19..20,
    },
    Token {
        kind: Whitespace,
        span: 20..21,
    },
    Token {
        kind: Equals,
        span: 21..22,
    },
    Token {
        kind: Newline,
        span: 22..23,
    },
    Token {
        kind: Comment,
        span: 23..29,
    },
    Token {
        kind: Newline,
        span: 29..31,
    },
    Token {
        kind: Comment,
        span: 31..36,
    },
    Token {
        kind: Newline,
        span: 36..37,
    },
    Token {
        kind: Whitespace,
        span: 37..38,
    },
    Token {
        kind: Eof,
        span: 38..38,
    },
]

"#]] . raw () ,) ; }
    };
}

all!()