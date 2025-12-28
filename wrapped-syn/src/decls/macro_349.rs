macro_rules! macro_349 {
    () => {
        ast_struct ! { # [doc = " A block of foreign items: `extern \"C\" { ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ItemForeignMod { pub attrs : Vec < Attribute >, pub unsafety : Option < Token ! [unsafe] >, pub abi : Abi , pub brace_token : token :: Brace , pub items : Vec < ForeignItem >, } }
    };
}

macro_349!()