// Generated macro for bad_comment (function)
macro_rules! Depcrate_lexer_testbad_comment {
() => {
// Module: crate::lexer::test
// Provides: {"bad_comment"}
// Dependencies: {}
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
