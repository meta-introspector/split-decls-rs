macro_rules! macro_749 {
    () => {
        ast_struct ! { # [doc = " A macro in the type position."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TypeMacro { pub mac : Macro , } }
    };
}

macro_749!()