macro_rules! macro_214 {
    () => {
        ast_struct ! { # [doc = " A square bracketed indexing expression: `vector[2]`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprIndex { pub attrs : Vec < Attribute >, pub expr : Box < Expr >, pub bracket_token : token :: Bracket , pub index : Box < Expr >, } }
    };
}

macro_214!();