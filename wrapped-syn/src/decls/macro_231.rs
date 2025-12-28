macro_rules! macro_231 {
    () => {
        ast_struct ! { # [doc = " A try block: `try { ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprTryBlock # full { pub attrs : Vec < Attribute >, pub try_token : Token ! [try] , pub block : Block , } }
    };
}

macro_231!()