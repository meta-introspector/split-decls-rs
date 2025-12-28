macro_rules! macro_216 {
    () => {
        ast_struct ! { # [doc = " A `let` guard: `let Some(x) = opt`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprLet # full { pub attrs : Vec < Attribute >, pub let_token : Token ! [let] , pub pat : Box < Pat >, pub eq_token : Token ! [=] , pub expr : Box < Expr >, } }
    };
}

macro_216!();