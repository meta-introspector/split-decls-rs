macro_rules! macro_217 {
    () => {
        ast_struct ! { # [doc = " A literal in place of an expression: `1`, `\"foo\"`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprLit { pub attrs : Vec < Attribute >, pub lit : Lit , } }
    };
}

macro_217!();