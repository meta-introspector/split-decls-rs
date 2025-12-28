macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! keylike {
    () => {
        deps!();
        # [test] fn keylike () { t ("foo" , str ! [[r#"
[
    Token {
        kind: Atom,
        span: 0..3,
    },
    Token {
        kind: Eof,
        span: 3..3,
    },
]

"#]] . raw () ,) ; t ("0bar" , str ! [[r#"
[
    Token {
        kind: Atom,
        span: 0..4,
    },
    Token {
        kind: Eof,
        span: 4..4,
    },
]

"#]] . raw () ,) ; t ("bar0" , str ! [[r#"
[
    Token {
        kind: Atom,
        span: 0..4,
    },
    Token {
        kind: Eof,
        span: 4..4,
    },
]

"#]] . raw () ,) ; t ("1234" , str ! [[r#"
[
    Token {
        kind: Atom,
        span: 0..4,
    },
    Token {
        kind: Eof,
        span: 4..4,
    },
]

"#]] . raw () ,) ; t ("a-b" , str ! [[r#"
[
    Token {
        kind: Atom,
        span: 0..3,
    },
    Token {
        kind: Eof,
        span: 3..3,
    },
]

"#]] . raw () ,) ; t ("a_B" , str ! [[r#"
[
    Token {
        kind: Atom,
        span: 0..3,
    },
    Token {
        kind: Eof,
        span: 3..3,
    },
]

"#]] . raw () ,) ; t ("-_-" , str ! [[r#"
[
    Token {
        kind: Atom,
        span: 0..3,
    },
    Token {
        kind: Eof,
        span: 3..3,
    },
]

"#]] . raw () ,) ; t ("___" , str ! [[r#"
[
    Token {
        kind: Atom,
        span: 0..3,
    },
    Token {
        kind: Eof,
        span: 3..3,
    },
]

"#]] . raw () ,) ; }
    };
}

keylike!()