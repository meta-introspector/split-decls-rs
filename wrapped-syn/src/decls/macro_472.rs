macro_rules! macro_472 {
    () => {
        ast_enum ! { # [doc = " A grouping token that surrounds a macro body: `m!(...)` or `m!{...}` or `m![...]`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub enum MacroDelimiter { Paren (Paren) , Brace (Brace) , Bracket (Bracket) , } }
    };
}

macro_472!()