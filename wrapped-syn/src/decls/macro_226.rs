macro_rules! macro_226 {
    () => {
        ast_struct ! { # [doc = " A referencing operation: `&a` or `&mut a`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprReference { pub attrs : Vec < Attribute >, pub and_token : Token ! [&] , pub mutability : Option < Token ! [mut] >, pub expr : Box < Expr >, } }
    };
}

macro_226!()