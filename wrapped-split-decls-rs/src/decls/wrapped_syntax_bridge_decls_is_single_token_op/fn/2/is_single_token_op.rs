use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
fn is_single_token_op (kind : SyntaxKind) -> bool { matches ! (kind , EQ | L_ANGLE | R_ANGLE | BANG | AMP | PIPE | TILDE | AT | DOT | COMMA | SEMICOLON | COLON | POUND | DOLLAR | QUESTION | PLUS | MINUS | STAR | SLASH | PERCENT | CARET | LIFETIME_IDENT) }
}