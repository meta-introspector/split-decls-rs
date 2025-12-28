macro_rules! macro_219 {
    () => {
        ast_struct ! { # [doc = " A macro invocation expression: `format!(\"{}\", q)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprMacro { pub attrs : Vec < Attribute >, pub mac : Macro , } }
    };
}

macro_219!()