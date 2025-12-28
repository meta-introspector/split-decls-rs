macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! basic_strings {
    () => {
        deps!();
        # [test] fn basic_strings () { t (r#""""# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..2,
    },
    Token {
        kind: Eof,
        span: 2..2,
    },
]

"#]] . raw () ,) ; t (r#""""""""# , str ! [[r#"
[
    Token {
        kind: MlBasicString,
        span: 0..6,
    },
    Token {
        kind: Eof,
        span: 6..6,
    },
]

"#]] . raw () ,) ; t (r#""a""# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..3,
    },
    Token {
        kind: Eof,
        span: 3..3,
    },
]

"#]] . raw () ,) ; t (r#""""a""""# , str ! [[r#"
[
    Token {
        kind: MlBasicString,
        span: 0..7,
    },
    Token {
        kind: Eof,
        span: 7..7,
    },
]

"#]] . raw () ,) ; t (r#""\t""# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..4,
    },
    Token {
        kind: Eof,
        span: 4..4,
    },
]

"#]] . raw () ,) ; t (r#""\u0000""# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..8,
    },
    Token {
        kind: Eof,
        span: 8..8,
    },
]

"#]] . raw () ,) ; t (r#""\U00000000""# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..12,
    },
    Token {
        kind: Eof,
        span: 12..12,
    },
]

"#]] . raw () ,) ; t (r#""\U000A0000""# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..12,
    },
    Token {
        kind: Eof,
        span: 12..12,
    },
]

"#]] . raw () ,) ; t (r#""\\t""# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..5,
    },
    Token {
        kind: Eof,
        span: 5..5,
    },
]

"#]] . raw () ,) ; t ("\"\t\"" , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..3,
    },
    Token {
        kind: Eof,
        span: 3..3,
    },
]

"#]] . raw () ,) ; t ("\"\"\"\n\t\"\"\"" , str ! [[r#"
[
    Token {
        kind: MlBasicString,
        span: 0..8,
    },
    Token {
        kind: Eof,
        span: 8..8,
    },
]

"#]] . raw () ,) ; t ("\"\"\"\\\n\"\"\"" , str ! [[r#"
[
    Token {
        kind: MlBasicString,
        span: 0..8,
    },
    Token {
        kind: Eof,
        span: 8..8,
    },
]

"#]] . raw () ,) ; t ("\"\"\"\\\n     \t   \t  \\\r\n  \t \n  \t \r\n\"\"\"" , str ! [[r#"
[
    Token {
        kind: MlBasicString,
        span: 0..34,
    },
    Token {
        kind: Eof,
        span: 34..34,
    },
]

"#]] . raw () ,) ; t (r#""\r""# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..4,
    },
    Token {
        kind: Eof,
        span: 4..4,
    },
]

"#]] . raw () ,) ; t (r#""\n""# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..4,
    },
    Token {
        kind: Eof,
        span: 4..4,
    },
]

"#]] . raw () ,) ; t (r#""\b""# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..4,
    },
    Token {
        kind: Eof,
        span: 4..4,
    },
]

"#]] . raw () ,) ; t (r#""a\fa""# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..6,
    },
    Token {
        kind: Eof,
        span: 6..6,
    },
]

"#]] . raw () ,) ; t (r#""\"a""# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..5,
    },
    Token {
        kind: Eof,
        span: 5..5,
    },
]

"#]] . raw () ,) ; t ("\"\"\"\na\"\"\"" , str ! [[r#"
[
    Token {
        kind: MlBasicString,
        span: 0..8,
    },
    Token {
        kind: Eof,
        span: 8..8,
    },
]

"#]] . raw () ,) ; t ("\"\"\"\n\"\"\"" , str ! [[r#"
[
    Token {
        kind: MlBasicString,
        span: 0..7,
    },
    Token {
        kind: Eof,
        span: 7..7,
    },
]

"#]] . raw () ,) ; t (r#""""a\"""b""""# , str ! [[r#"
[
    Token {
        kind: MlBasicString,
        span: 0..12,
    },
    Token {
        kind: Eof,
        span: 12..12,
    },
]

"#]] . raw () ,) ; t (r#""\a"# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..3,
    },
    Token {
        kind: Eof,
        span: 3..3,
    },
]

"#]] . raw () ,) ; t ("\"\\\n" , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..2,
    },
    Token {
        kind: Newline,
        span: 2..3,
    },
    Token {
        kind: Eof,
        span: 3..3,
    },
]

"#]] . raw () ,) ; t ("\"\\\r\n" , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..3,
    },
    Token {
        kind: Newline,
        span: 3..4,
    },
    Token {
        kind: Eof,
        span: 4..4,
    },
]

"#]] . raw () ,) ; t ("\"\\" , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..2,
    },
    Token {
        kind: Eof,
        span: 2..2,
    },
]

"#]] . raw () ,) ; t ("\"\u{0}" , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..2,
    },
    Token {
        kind: Eof,
        span: 2..2,
    },
]

"#]] . raw () ,) ; t (r#""\U00""# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..6,
    },
    Token {
        kind: Eof,
        span: 6..6,
    },
]

"#]] . raw () ,) ; t (r#""\U00"# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..5,
    },
    Token {
        kind: Eof,
        span: 5..5,
    },
]

"#]] . raw () ,) ; t (r#""\uD800"# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..7,
    },
    Token {
        kind: Eof,
        span: 7..7,
    },
]

"#]] . raw () ,) ; t (r#""\UFFFFFFFF"# , str ! [[r#"
[
    Token {
        kind: BasicString,
        span: 0..11,
    },
    Token {
        kind: Eof,
        span: 11..11,
    },
]

"#]] . raw () ,) ; }
    };
}

basic_strings!();