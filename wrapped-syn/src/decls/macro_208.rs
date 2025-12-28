macro_rules! macro_208 {
    () => {
        ast_struct ! { # [doc = " A const block: `const { ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprConst # full { pub attrs : Vec < Attribute >, pub const_token : Token ! [const] , pub block : Block , } }
    };
}

macro_208!();