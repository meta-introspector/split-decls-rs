// Generated macro for rule (function)
macro_rules! Depcrate_parserrule {
() => {
// Module: crate::parser
// Provides: {"rule"}
// Dependencies: {}
fn rule (p : & mut Parser) -> Result < Rule > { if let Some (lexer :: Token { kind : TokenKind :: Pipe , loc }) = p . peek () { bail ! (* loc , "The first element in a sequence of productions or alternatives \
            must not have a leading pipe (`|`)") ; } let lhs = seq_rule (p) ? ; let mut alt = vec ! [lhs] ; while let Some (token) = p . peek () { if token . kind != TokenKind :: Pipe { break ; } p . bump () ? ; let rule = seq_rule (p) ? ; alt . push (rule) } let res = if alt . len () == 1 { alt . pop () . unwrap () } else { Rule :: Alt (alt) } ; Ok (res) }
};
}
