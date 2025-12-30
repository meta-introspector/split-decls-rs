// Generated macro for atom_rule (function)
macro_rules! Depcrate_parseratom_rule {
() => {
// Module: crate::parser
// Provides: {"atom_rule"}
// Dependencies: {}
fn atom_rule (p : & mut Parser) -> Result < Rule > { match opt_atom_rule (p) ? { Some (it) => Ok (it) , None => { let token = p . bump () ? ; bail ! (token . loc , "unexpected token") } } }
};
}
