// Generated macro for is_single_token_op (function)
macro_rules! Depcrateis_single_token_op {
() => {
// Module: crate
// Provides: {"is_single_token_op"}
// Dependencies: {}
fn is_single_token_op (kind : SyntaxKind) -> bool { matches ! (kind , EQ | L_ANGLE | R_ANGLE | BANG | AMP | PIPE | TILDE | AT | DOT | COMMA | SEMICOLON | COLON | POUND | DOLLAR | QUESTION | PLUS | MINUS | STAR | SLASH | PERCENT | CARET | LIFETIME_IDENT) }
};
}
