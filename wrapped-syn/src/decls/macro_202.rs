macro_rules! macro_202 {
    () => {
        ast_struct ! { # [doc = " A binary operation: `a + b`, `a += b`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprBinary { pub attrs : Vec < Attribute >, pub left : Box < Expr >, pub op : BinOp , pub right : Box < Expr >, } }
    };
}

macro_202!()