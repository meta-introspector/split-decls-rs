macro_rules! macro_750 {
    () => {
        ast_struct ! { # [doc = " The never type: `!`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeNever { pub bang_token : Token ! [!] , } }
    };
}

macro_750!();