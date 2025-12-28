macro_rules! macro_234 {
    () => {
        ast_struct ! { # [doc = " An unsafe block: `unsafe { ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprUnsafe # full { pub attrs : Vec < Attribute >, pub unsafe_token : Token ! [unsafe] , pub block : Block , } }
    };
}

macro_234!();