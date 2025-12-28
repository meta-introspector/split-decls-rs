macro_rules! macro_200 {
    () => {
        ast_struct ! { # [doc = " An async block: `async { ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprAsync # full { pub attrs : Vec < Attribute >, pub async_token : Token ! [async] , pub capture : Option < Token ! [move] >, pub block : Block , } }
    };
}

macro_200!()