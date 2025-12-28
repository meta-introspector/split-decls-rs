macro_rules! deps {
    () => {
        Value!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl Parse for Value { fn parse (input : ParseStream < '_ >) -> Result < Self > { let expr : Expr = input . parse () ? ; match & expr { Expr :: Lit (expr) => { if let Lit :: Str (lit) = & expr . lit { return Ok (Value :: String (lit . clone ())) ; } } Expr :: Macro (expr) => { if expr . mac . path . is_ident ("env") && let Ok (lit) = expr . mac . parse_body () { return Ok (Value :: Env (lit , expr . mac . clone ())) ; } } _ => { } } Ok (Value :: Unsupported (expr)) } }
    };
}

impl_129!();