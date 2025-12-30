// Generated macro for node (function)
macro_rules! Depcrate_parsernode {
() => {
// Module: crate::parser
// Provides: {"node"}
// Dependencies: {}
fn node (p : & mut Parser) -> Result < () > { let token = p . bump () ? ; let node = match token . kind { TokenKind :: Node (it) => p . intern_node (it) , _ => bail ! (token . loc , "expected ident") , } ; p . expect (TokenKind :: Eq , "=") ? ; if ! matches ! (p . grammar [node] . rule , DUMMY_RULE) { bail ! (token . loc , "duplicate rule: `{}`" , p . grammar [node] . name) } let rule = rule (p) ? ; p . grammar . nodes [node . 0] . rule = rule ; Ok (()) }
};
}
