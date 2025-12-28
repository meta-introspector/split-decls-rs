macro_rules! macro_206 {
    () => {
        ast_struct ! { # [doc = " A cast expression: `foo as f64`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprCast { pub attrs : Vec < Attribute >, pub expr : Box < Expr >, pub as_token : Token ! [as] , pub ty : Box < Type >, } }
    };
}

macro_206!()