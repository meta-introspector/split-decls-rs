macro_rules! macro_471 {
    () => {
        ast_struct ! { # [doc = " A macro invocation: `println!(\"{}\", mac)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct Macro { pub path : Path , pub bang_token : Token ! [!] , pub delimiter : MacroDelimiter , pub tokens : TokenStream , } }
    };
}

macro_471!();