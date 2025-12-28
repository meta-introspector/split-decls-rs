macro_rules! macro_233 {
    () => {
        ast_struct ! { # [doc = " A unary operation: `!x`, `*x`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprUnary { pub attrs : Vec < Attribute >, pub op : UnOp , pub expr : Box < Expr >, } }
    };
}

macro_233!()