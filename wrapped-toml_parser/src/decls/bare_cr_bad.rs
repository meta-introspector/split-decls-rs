macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! bare_cr_bad {
    () => {
        deps!();
        # [test] fn bare_cr_bad () { t ("\r" , str ! [[r#"
[
    Token {
        kind: Newline,
        span: 0..1,
    },
    Token {
        kind: Eof,
        span: 1..1,
    },
]

"#]] . raw () ,) ; t ("'\n" , str ! [[r#"
[
    Token {
        kind: LiteralString,
        span: 0..1,
    },
    Token {
        kind: Newline,
        span: 1..2,
    },
    Token {
        kind: Eof,
        span: 2..2,
    },
]

"#]] . raw () ,) ; t ("'\u{0}" , str ! [[r#"
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

"#]] . raw () ,) ; t ("'" , str ! [[r#"
[
    Token {
        kind: LiteralString,
        span: 0..1,
    },
    Token {
        kind: Eof,
        span: 1..1,
    },
]

"#]] . raw () ,) ; t ("\u{0}" , str ! [[r#"
[
    Token {
        kind: Atom,
        span: 0..1,
    },
    Token {
        kind: Eof,
        span: 1..1,
    },
]

"#]] . raw () ,) ; }
    };
}

bare_cr_bad!();