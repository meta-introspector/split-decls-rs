macro_rules! deps {
    () => {
        Token!();
    };
}

macro_rules! bad_comment {
    () => {
        deps!();
        # [test] fn bad_comment () { t ("#\u{0}" , str ! [[r#"
[
    Token {
        kind: Comment,
        span: 0..2,
    },
    Token {
        kind: Eof,
        span: 2..2,
    },
]

"#]] . raw () ,) ; }
    };
}

bad_comment!()