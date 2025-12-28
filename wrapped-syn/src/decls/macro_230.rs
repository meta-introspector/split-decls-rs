macro_rules! macro_230 {
    () => {
        ast_struct ! { # [doc = " A try-expression: `expr?`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprTry # full { pub attrs : Vec < Attribute >, pub expr : Box < Expr >, pub question_token : Token ! [?] , } }
    };
}

macro_230!()