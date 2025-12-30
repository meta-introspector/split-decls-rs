// Generated macro for advance (function)
macro_rules! Depcrate_lexeradvance {
() => {
// Module: crate::lexer
// Provides: {"advance"}
// Dependencies: {}
fn advance (input : & mut & str) -> Result < TokenKind > { let mut chars = input . chars () ; let c = chars . next () . unwrap () ; let res = match c { '=' => TokenKind :: Eq , '*' => TokenKind :: Star , '?' => TokenKind :: QMark , '(' => TokenKind :: LParen , ')' => TokenKind :: RParen , '|' => TokenKind :: Pipe , ':' => TokenKind :: Colon , '\'' => { let mut buf = String :: new () ; loop { match chars . next () { None => bail ! ("unclosed token literal") , Some ('\\') => match chars . next () { Some (c) if is_escapable (c) => buf . push (c) , _ => bail ! ("invalid escape in token literal") , } , Some ('\'') => break , Some (c) => buf . push (c) , } } TokenKind :: Token (buf) } c if is_ident_char (c) => { let mut buf = String :: new () ; buf . push (c) ; loop { match chars . clone () . next () { Some (c) if is_ident_char (c) => { chars . next () ; buf . push (c) ; } _ => break , } } TokenKind :: Node (buf) } '\r' => bail ! ("unexpected `\\r`, only Unix-style line endings allowed") , c => bail ! ("unexpected character: `{}`" , c) , } ; * input = chars . as_str () ; Ok (res) }
};
}
