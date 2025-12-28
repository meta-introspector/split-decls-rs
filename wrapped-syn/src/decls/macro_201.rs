macro_rules! macro_201 {
    () => {
        ast_struct ! { # [doc = " An await expression: `fut.await`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprAwait # full { pub attrs : Vec < Attribute >, pub base : Box < Expr >, pub dot_token : Token ! [.] , pub await_token : Token ! [await] , } }
    };
}

macro_201!()