// Generated macro for opt_atom_rule (function)
macro_rules! Depcrate_parseropt_atom_rule {
() => {
// Module: crate::parser
// Provides: {"opt_atom_rule"}
// Dependencies: {}
fn opt_atom_rule (p : & mut Parser) -> Result < Option < Rule > > { let token = match p . peek () { Some (it) => it , None => return Ok (None) , } ; let mut res = match & token . kind { TokenKind :: Node (name) => { if let Some (lookahead) = p . peek_n (1) { match lookahead . kind { TokenKind :: Eq => return Ok (None) , TokenKind :: Colon => { let label = name . clone () ; p . bump () ? ; p . bump () ? ; let rule = atom_rule (p) ? ; let res = Rule :: Labeled { label , rule : Box :: new (rule) } ; return Ok (Some (res)) ; } _ => () , } } match p . peek_n (1) { Some (token) if token . kind == TokenKind :: Eq => return Ok (None) , _ => () , } let name = name . clone () ; p . bump () ? ; let node = p . intern_node (name) ; Rule :: Node (node) } TokenKind :: Token (name) => { let name = name . clone () ; p . bump () ? ; let token = p . intern_token (name) ; Rule :: Token (token) } TokenKind :: LParen => { p . bump () ? ; let rule = rule (p) ? ; p . expect (TokenKind :: RParen , ")") ? ; rule } _ => return Ok (None) , } ; if let Some (token) = p . peek () { match & token . kind { TokenKind :: QMark => { p . bump () ? ; res = Rule :: Opt (Box :: new (res)) ; } TokenKind :: Star => { p . bump () ? ; res = Rule :: Rep (Box :: new (res)) ; } _ => () , } } Ok (Some (res)) }
};
}
