macro_rules! macro_236 {
    () => {
        ast_struct ! { # [doc = " A yield expression: `yield expr`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprYield # full { pub attrs : Vec < Attribute >, pub yield_token : Token ! [yield] , pub expr : Option < Box < Expr >>, } }
    };
}

macro_236!();