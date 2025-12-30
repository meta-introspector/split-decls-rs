// Generated macro for impl_217 (impl)
macro_rules! Depcrate_symbolsimpl_217 {
() => {
// Module: crate::symbols
// Provides: {"impl_217"}
// Dependencies: {}
impl Parse for Value { fn parse (input : ParseStream < '_ >) -> Result < Self > { let expr : Expr = input . parse () ? ; match & expr { Expr :: Lit (expr) => { if let Lit :: Str (lit) = & expr . lit { return Ok (Value :: String (lit . clone ())) ; } } Expr :: Macro (expr) => { if expr . mac . path . is_ident ("env") && let Ok (lit) = expr . mac . parse_body () { return Ok (Value :: Env (lit , expr . mac . clone ())) ; } } _ => { } } Ok (Value :: Unsupported (expr)) } }
};
}
