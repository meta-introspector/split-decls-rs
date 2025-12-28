macro_rules! macro_203 {
    () => {
        ast_struct ! { # [doc = " A blocked scope: `{ ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprBlock # full { pub attrs : Vec < Attribute >, pub label : Option < Label >, pub block : Block , } }
    };
}

macro_203!();