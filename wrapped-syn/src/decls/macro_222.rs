macro_rules! macro_222 {
    () => {
        ast_struct ! { # [doc = " A parenthesized expression: `(a + b)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprParen { pub attrs : Vec < Attribute >, pub paren_token : token :: Paren , pub expr : Box < Expr >, } }
    };
}

macro_222!()