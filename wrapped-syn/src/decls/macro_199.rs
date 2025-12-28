macro_rules! macro_199 {
    () => {
        ast_struct ! { # [doc = " An assignment expression: `a = compute()`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprAssign # full { pub attrs : Vec < Attribute >, pub left : Box < Expr >, pub eq_token : Token ! [=] , pub right : Box < Expr >, } }
    };
}

macro_199!()