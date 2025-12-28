macro_rules! macro_73 {
    () => {
        ast_struct ! { # [doc = " A structured list within an attribute, like `derive(Copy, Clone)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct MetaList { pub path : Path , pub delimiter : MacroDelimiter , pub tokens : TokenStream , } }
    };
}

macro_73!();