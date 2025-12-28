macro_rules! macro_758 {
    () => {
        ast_struct ! { # [doc = " The binary interface of a function: `extern \"C\"`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct Abi { pub extern_token : Token ! [extern] , pub name : Option < LitStr >, } }
    };
}

macro_758!()