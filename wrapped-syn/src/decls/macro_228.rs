macro_rules! macro_228 {
    () => {
        ast_struct ! { # [doc = " A `return`, with an optional value to be returned."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprReturn # full { pub attrs : Vec < Attribute >, pub return_token : Token ! [return] , pub expr : Option < Box < Expr >>, } }
    };
}

macro_228!()