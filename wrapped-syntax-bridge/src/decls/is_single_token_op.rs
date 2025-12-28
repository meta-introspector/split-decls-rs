macro_rules! is_single_token_op {
    () => {
        fn is_single_token_op (kind : SyntaxKind) -> bool { matches ! (kind , EQ | L_ANGLE | R_ANGLE | BANG | AMP | PIPE | TILDE | AT | DOT | COMMA | SEMICOLON | COLON | POUND | DOLLAR | QUESTION | PLUS | MINUS | STAR | SLASH | PERCENT | CARET | LIFETIME_IDENT) }
    };
}

is_single_token_op!();