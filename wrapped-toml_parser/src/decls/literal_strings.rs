macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! literal_strings {
    () => {
        deps!();
        # [test] fn literal_strings () { t ("''" , str ! [[r#"
[
    Token {
        kind: LiteralString,
        span: 0..2,
    },
    Token {
        kind: Eof,
        span: 2..2,
    },
]

"#]] . raw () ,) ; t ("''''''" , str ! [[r#"
[
    Token {
        kind: MlLiteralString,
        span: 0..6,
    },
    Token {
        kind: Eof,
        span: 6..6,
    },
]

"#]] . raw () ,) ; t ("'''\n'''" , str ! [[r#"
[
    Token {
        kind: MlLiteralString,
        span: 0..7,
    },
    Token {
        kind: Eof,
        span: 7..7,
    },
]

"#]] . raw () ,) ; t ("'a'" , str ! [[r#"
[
    Token {
        kind: LiteralString,
        span: 0..3,
    },
    Token {
        kind: Eof,
        span: 3..3,
    },
]

"#]] . raw () ,) ; t ("'\"a'" , str ! [[r#"
[
    Token {
        kind: LiteralString,
        span: 0..4,
    },
    Token {
        kind: Eof,
        span: 4..4,
    },
]

"#]] . raw () ,) ; t ("''''a'''" , str ! [[r#"
[
    Token {
        kind: MlLiteralString,
        span: 0..8,
    },
    Token {
        kind: Eof,
        span: 8..8,
    },
]

"#]] . raw () ,) ; t ("'''\n'a\n'''" , str ! [[r#"
[
    Token {
        kind: MlLiteralString,
        span: 0..10,
    },
    Token {
        kind: Eof,
        span: 10..10,
    },
]

"#]] . raw () ,) ; t ("'''a\n'a\r\n'''" , str ! [[r#"
[
    Token {
        kind: MlLiteralString,
        span: 0..12,
    },
    Token {
        kind: Eof,
        span: 12..12,
    },
]

"#]] . raw () ,) ; }
    };
}

literal_strings!()